// use rusoto core and s3
extern crate rusoto_core;
extern crate rusoto_s3;

use futures::{Future, Stream};
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest, CreateBucketRequest,
                SelectObjectContentRequest, InputSerialization, JSONInput, OutputSerialization,
                JSONOutput};

fn main() {
    println!("Starting up");
    
    let client = S3Client::new(Region::UsEast1);
    // create a bucket
    let test_bucket = "rusoto-test-bucket-0123456";
    let create_bucket_req = CreateBucketRequest { bucket: test_bucket.to_owned(), ..Default::default() };
    client.create_bucket(create_bucket_req).sync().expect("Couldn't create bucket");

    // create a sample json file with multiple fields
    let test_json = String::from(r#"{"foo": 1, "bar": 2}"#);

    // upload the file
    let contents: Vec<u8> = test_json.into_bytes();
    let req = PutObjectRequest {
        bucket: test_bucket.to_owned(),
        key: "select-sample.json".to_owned(),
        body: Some(contents.into()),
        ..Default::default()
    };
    client.put_object(req).sync().expect("Couldn't PUT object");

    // run s3 select query on it, verify it only has the fields we requested
    // see https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectSELECTContent.html .
    let input_serialization = InputSerialization { 
        json: Some(JSONInput {
            type_: Some("Document".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let output_serialization = OutputSerialization { ..Default::default() };
    
    let get_req_select = SelectObjectContentRequest {
        bucket: test_bucket.to_owned(),
        key: "select-sample.json".to_owned(),
        expression: "SELECT s._1 FROM S3Object s".to_owned(),
        expression_type: "SQL".to_owned(),
        input_serialization: input_serialization,
        output_serialization: output_serialization,
        ..Default::default()
    };
    let select_file_response = client.select_object_content(get_req_select).sync().expect("Couldn't download file");
    let stream_select = select_file_response.payload.unwrap();
    let body_select = stream_select.concat2().wait().unwrap();
    println!("body_select is '{}'", String::from_utf8(body_select).unwrap());

    // run regular s3 get_object, see it has all the fields
    let get_req = GetObjectRequest {
        bucket: test_bucket.to_owned(),
        key: "select-sample.json".to_owned(),
        ..Default::default()
    };
    let whole_file_response = client.get_object(get_req).sync().expect("Couldn't download file");
    let stream = whole_file_response.body.unwrap();
    let body = stream.concat2().wait().unwrap();
    println!("body is '{}'", String::from_utf8(body).unwrap());

    // delete the bucket
    println!("All done");
}
