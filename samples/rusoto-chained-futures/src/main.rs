extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate tokio_core;

use futures::future::Future;
use tokio_core::reactor::Core;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableInput, DynamoDb, DynamoDbClient, GetItemInput,
    KeySchemaElement, ProvisionedThroughput, UpdateItemInput,
};
use std::collections::HashMap;

// Use local dynamodb for testing this out
fn main() {
    // Create custom Region
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };

    let client = DynamoDbClient::new(region);

    // future for creating a table
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

    let create_table_future = client
        .create_table(make_table_request)
        .map(|_r| ())
        .map_err(|e| panic!("no: {}", e));

    // future for upserting an entry
    let mut item = HashMap::new();
    item.insert(
        "foo_name".to_string(),
        AttributeValue {
            s: Some("baz".to_string()),
            ..Default::default()
        },
    );
    let add_item = UpdateItemInput {
        key: item.clone(), // cloning so we can reuse the item we're looking for in the GET part
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };

    let upsert_item_future = client
        .update_item(add_item)
        .map(|_| ())
        .map_err(|e| panic!("Couldn't upsert item: {}", e));

    // future for getting the entry
    let get_item_request = GetItemInput {
        key: item,
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };
    let item_from_dynamo_future = client.get_item(get_item_request);

    // tie them together
    let chained_futures = create_table_future.and_then(|_| {
        upsert_item_future.and_then(|_| {
            item_from_dynamo_future.map(|i| println!("Got item back: {:?}", i)).map_err(|e| panic!("Couldn't get item: {}", e))
        })
    });

    // This tokio core will run our futures to completion:
    let mut core = Core::new().unwrap();

    // run 'em
    core.run(chained_futures).unwrap();
}
