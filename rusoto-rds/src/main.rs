extern crate rusoto;

use std::{thread, time};

use rusoto::rds::{RdsClient, CreateDBInstanceMessage, DescribeDBInstancesMessage};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

fn main() {
    let database_instance_name = "rusototester2";
    let credentials = DefaultCredentialsProvider::new().unwrap();

    // Security groups in the default VPC will need modification to let you access this from the internet:

    let rds_client = RdsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
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
    let db_creation_result = rds_client.create_db_instance(&create_db_instance_request).unwrap();
    println!("Created! \n\n{:?}", db_creation_result);

    // The endpoint isn't available until the DB is created, let's wait for it:
    let describe_instances_request = DescribeDBInstancesMessage {
        db_instance_identifier: Some(database_instance_name.to_string()),
        ..Default::default()
    };

    let endpoint : rusoto::rds::Endpoint;
    let ten_seconds = time::Duration::from_millis(10000);
    loop {
        match rds_client.describe_db_instances(&describe_instances_request).unwrap().db_instances.unwrap()[0].endpoint {
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

    let endpoint_address = endpoint.address.unwrap();
    let endpoint_port = endpoint.port.unwrap();
    println!("\n\nendpoint: {:?}", format!("{}:{}", endpoint_address, endpoint_port));
}
