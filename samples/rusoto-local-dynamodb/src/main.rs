extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableRequest, DynamoDb, DynamoDbClient, GetItemRequest,
    KeySchemaElement, ListTablesRequest, ProvisionedThroughput, UpdateItemRequest,
};
use std::collections::HashMap;

fn main() {
    // Create custom Region
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };

    let client = DynamoDbClient::new(region);

    // List tables
    let list_tables_request = ListTablesRequest::default();
    let tables = client.list_tables(list_tables_request).sync();

    println!("Tables found: {:?}", tables);

    // Create table if not present
    match tables {
        Ok(ts) => match ts.table_names {
            Some(t) => {
                println!("Found a table, not creating one.");
                if t.len() == 0 {
                    println!("Creating a table.");

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
                    let make_table_request = CreateTableRequest {
                        table_name: "a-testing-table".to_string(),
                        attribute_definitions: vec![attribute_def],
                        key_schema: vec![k_schema],
                        provisioned_throughput: Some(p_throughput),
                        ..Default::default()
                    };

                    client
                        .create_table(make_table_request)
                        .sync()
                        .expect("Table creation should work.");

                    println!("Table created!");
                }
            }
            None => (), // Rusoto currently returns an empty array if the table list is empty
        },
        Err(_) => println!("Error getting tables."),
    }

    // Add an entry, overwriting with new data if present:
    let mut item = HashMap::new();
    item.insert(
        "foo_name".to_string(),
        AttributeValue {
            s: Some("baz".to_string()),
            ..Default::default()
        },
    );
    let add_item = UpdateItemRequest {
        key: item.clone(), // cloning so we can reuse the item we're looking for in the GET part
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };

    client
        .update_item(add_item)
        .sync()
        .expect("Item failed to add");
    println!("Key added or updated!");

    // Retrieve the entry
    let get_item_request = GetItemRequest {
        key: item,
        table_name: "a-testing-table".to_string(),
        ..Default::default()
    };
    let item_from_dynamo = client
        .get_item(get_item_request)
        .sync()
        .expect("Should be able to get item");

    println!("Got this item: {:?}\n", item_from_dynamo.item);
}
