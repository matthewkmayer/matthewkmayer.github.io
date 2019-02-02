extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate tokio_core;

use futures::future::Future;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableError, CreateTableInput, CreateTableOutput,
    DynamoDb, DynamoDbClient, GetItemError, GetItemInput, GetItemOutput, KeySchemaElement,
    ProvisionedThroughput,
};
use std::collections::HashMap;
use tokio_core::reactor::Core;

fn main() {
    let item = make_item();
    let client = get_dynamodb_local_client();
    // This tokio core will run our futures to completion:
    let mut core = Core::new().unwrap();

    let make_table_future_the_second = create_table_future_with_error_handling(&client);
    let get_item_future = make_get_item_future_with_error_handling(&client, &item);

    let chained = make_table_future_the_second
        .map_err(|e| {
            println!("We could do something with the table creation error here.");
            // We need to make any error return look like the innermost error returned.
            // Let's create a GetItemError:
            GetItemError::InternalServerError(format!(
                "Actually from us! Real error from attempting to making the table: {}",
                e
            ))
        })
        .and_then(|r| {
            // If we're here, we successfully made the new table.
            // r will be the CreateTableOutput:
            println!("r is {:?}\n\n", r);

            // Finally, call the get_item_future and return the Result<GetItemOutput, GetItemError>
            // and we can match on that later.
            get_item_future
        });

    // `core.run(chained)` will look like the innermost future we called, which will get an item.
    // This is just like a synchronous Rusoto call of client.get_item(get_item_request).sync():
    let chained_with_failure_handling = match core.run(chained) {
        Ok(result) => format!("Got item: {:?}", result),
        Err(e) => format!("Didn't get item: {}", e),
    };

    // Since Rusoto returns success on fetching an item that doesn't exist, we'll see a blank item:
    // `GetItemOutput { consumed_capacity: None, item: None }`
    println!(
        "chained_with_failure_handling is {}",
        chained_with_failure_handling
    );

    println!("Done");
}

fn create_table_future_with_error_handling(
    client: &DynamoDbClient,
) -> impl Future<Item = CreateTableOutput, Error = CreateTableError> {
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

fn make_get_item_future_with_error_handling(
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
