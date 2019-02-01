extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate tokio_core;

use futures::future::Future;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableInput, CreateTableOutput, DynamoDb,
    DynamoDbClient, GetItemError, GetItemInput, GetItemOutput, KeySchemaElement,
    ProvisionedThroughput, UpdateItemInput, UpdateItemOutput,
};
use std::collections::HashMap;
use tokio_core::reactor::Core;

fn main() {
    let item = make_item();
    let client = get_dynamodb_local_client();

    let create_table_future = make_create_table_future(&client);
    let upsert_item_future = make_upsert_item_future(&client, &item);
    let item_from_dynamo_future = make_get_item_future(&client, &item);

    // tie them together
    let chained_futures = create_table_future
        .then(|_| upsert_item_future)
        .then(|_| item_from_dynamo_future);

    // This tokio core will run our futures to completion:
    let mut core = Core::new().unwrap();

    // run 'em
    let item_from_dynamo = match core.run(chained_futures) {
        Ok(item) => item,
        Err(_) => panic!("Error completing futures: {}"),
    };

    println!("item_from_dynamo is {:?}", item_from_dynamo);
    println!("Done");
}

fn make_create_table_future(client: &DynamoDbClient) -> impl Future<Item = CreateTableOutput> {
    let attribute_def = AttributeDefinition {
        attribute_name: "foo_name".to_string(),
        attribute_type: "S".to_string(),
    };
    let k_schema = KeySchemaElement {
        attribute_name: "foo_name".to_string(),
        key_type: "HASH".to_string(), // case sensitive
    };
    let p_throughput = ProvisionedThroughput {
        read_capacity_units: 1,
        write_capacity_units: 1,
    };
    let make_table_request = CreateTableInput {
        table_name: "a-testing-table".to_string(),
        attribute_definitions: vec![attribute_def],
        key_schema: vec![k_schema],
        provisioned_throughput: p_throughput,
        ..Default::default()
    };

    client.create_table(make_table_request)
}

fn make_upsert_item_future(
    client: &DynamoDbClient,
    item: &HashMap<String, AttributeValue>,
) -> impl Future<Item = UpdateItemOutput> {
    let add_item = UpdateItemInput {
        key: item.clone(),
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };

    client.update_item(add_item)
}

fn make_get_item_future(
    client: &DynamoDbClient,
    item: &HashMap<String, AttributeValue>,
) -> impl Future<Item = GetItemOutput, Error = GetItemError> {
    // future for getting the entry
    let get_item_request = GetItemInput {
        key: item.clone(),
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };
    client.get_item(get_item_request)
}

fn make_item() -> HashMap<String, AttributeValue> {
    let item_key = "foo_name";
    let mut item = HashMap::new();
    item.insert(
        item_key.to_string(),
        AttributeValue {
            s: Some("baz".to_string()),
            ..Default::default()
        },
    );

    item
}

fn get_dynamodb_local_client() -> DynamoDbClient {
    // Create custom Region
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };

    DynamoDbClient::new(region)
}