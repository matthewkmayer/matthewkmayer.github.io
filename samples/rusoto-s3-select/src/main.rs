// use rusoto core and s3
extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest, CreateBucketRequest};
// use rusoto_core::credential::{AwsCredentials, DefaultCredentialsProvider};

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

    // run regular s3 get_object, see it has all the fields

    
    // delete the bucket
    println!("All done");
}
