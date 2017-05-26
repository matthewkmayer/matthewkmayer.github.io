extern crate rusoto_core;
extern crate rusoto_rds;

use std::{thread, time};

use rusoto_rds::{Rds, RdsClient, CreateDBInstanceMessage, DescribeDBInstancesMessage};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

fn main() {
    let database_instance_name = "rusototester2";
    let credentials = DefaultCredentialsProvider::new().expect("Couldn't create AWS credentials provider.");

    // Security groups in the default VPC will need modification to let you access this from the internet:

    let rds_client = RdsClient::new(default_tls_client().expect("Couldn't get default TLS client"),
                                    credentials, Region::UsEast1);
    let create_db_instance_request = CreateDBInstanceMessage {
        allocated_storage: Some(5),
        backup_retention_period: Some(0),
        db_instance_identifier: database_instance_name.to_string(),
        db_instance_class: "db.t2.micro".to_string(),
        // name and login details should match `.env` in rusoto-rocket
        master_user_password: Some("TotallySecurePassword501".to_string()),
        master_username: Some("masteruser".to_string()),
        db_name: Some("rusotodb".to_string()),
        engine: "postgres".to_string(),
        multi_az: Some(false),
        ..Default::default()
    };

    println!("Going to make the database instance.");
    let db_call_result = rds_client.create_db_instance(&create_db_instance_request);
    if db_call_result.is_err() {
        // This `unwrap` on the `err()` call will show us the error we know is there:
        println!("Didn't successfully make the DB instance.  Error: {}", db_call_result.err().unwrap());
        // Since it didn't succeed, we can look at the error and see if we should retry or not.
        // For our sample, we'll panic.
        panic!("Error making DB instance creation request.");
    }
    // db_call_result is `ok`, we can unwrap without it panicking:
    let db_creation_result = db_call_result.unwrap();
    println!("Created! \n\n{:?}", db_creation_result);

    // The endpoint isn't available until the DB is created, let's wait for it:
    let describe_instances_request = DescribeDBInstancesMessage {
        db_instance_identifier: Some(database_instance_name.to_string()),
        ..Default::default()
    };

    let endpoint : rusoto_rds::Endpoint;
    let ten_seconds = time::Duration::from_millis(10000);
    loop {
        match rds_client.describe_db_instances(&describe_instances_request)
            .expect("Error sending describe instances request")
            .db_instances.expect("Didn't get a result for DB instances request")[0].endpoint {
            Some(ref endpoint_result) => {
                endpoint = endpoint_result.clone();
                break;
            },
            None => {
                println!("Waiting for db to be available...");
                thread::sleep(ten_seconds);
                continue;
            },
        };
    }

    let endpoint_address = endpoint.address.expect("Endpoint address not available");
    let endpoint_port = endpoint.port.expect("Endpoint port not available");
    println!("\n\nendpoint: {:?}", format!("{}:{}", endpoint_address, endpoint_port));
}
