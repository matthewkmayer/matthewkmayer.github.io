+++
date = "2017-05-14T21:28:53-07:00"
draft = false
title = "Rusoto RDS walkthrough"
+++

Let's tie some great Rust crates together!  In this walkthrough, we'll use [Rusoto](https://github.com/rusoto/rusoto) to create a Postgres RDS database instance,
[Rocket.rs](https://github.com/SergioBenitez/Rocket) to make a web server and [Diesel](https://github.com/diesel-rs/diesel) to talk to the database on AWS to make a proof of concept hit counter.

<!--more-->

## Walkthrough overview

There are two projects in this walkthrough.  First is [rusoto-rds](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rds).  This creates the Amazon Web Services (AWS) Relational Database Service (RDS) instance and should be run first.

The second is [rusoto-rocket](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rocket).  This is the Rocket web service sample.  It uses Diesel and Rocket to have a web site that connects to the RDS instance created in `rusoto-rds` and demonstrates a hit counter.

## Prerequisites

### Rocket

Starting with the Rocket web site, we'll need to use Rust nightly.  This walkthrough uses `rustc 1.18.0-nightly (036983201 2017-04-26)`.  To switch to that nightly release, run `rustup default nightly-2017-04-26`.  The output of that command should look like this:

```bash
info: syncing channel updates for 'nightly-2017-04-26-x86_64-apple-darwin'
info: downloading component 'rustc'
 42.3 MiB /  42.3 MiB (100 %) 1014.4 KiB/s ETA:   0 s                
info: downloading component 'rust-std'
 58.2 MiB /  58.2 MiB (100 %)   1.4 MiB/s ETA:   0 s                
info: downloading component 'cargo'
  3.6 MiB /   3.6 MiB (100 %)   1.1 MiB/s ETA:   0 s                
info: downloading component 'rust-docs'
 11.5 MiB /  11.5 MiB (100 %)   1.1 MiB/s ETA:   0 s                
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: default toolchain set to 'nightly-2017-04-26-x86_64-apple-darwin'

  nightly-2017-04-26-x86_64-apple-darwin installed - rustc 1.18.0-nightly (2b4c91158 2017-04-25)
```

Verify `rustc` is using the right version:

```bash
$ rustc --version
rustc 1.18.0-nightly (2b4c91158 2017-04-25)
```

### Diesel

To set up Diesel, we'll need to [install Postgres](https://wiki.postgresql.org/wiki/Detailed_installation_guides) to get the required libraries for Diesel CLI.  The Postgres service doesn't have to be running for this walkthrough.

Then install the Diesel CLI tool with the Postgres extensions: 

`cargo install diesel_cli --features "postgres" --no-default-features`.

### Rusoto

For the AWS portions of this walkthrough, ensure AWS access keys are available either in environment variables or AWS credentials file.

## Creating a Postgres RDS instance

See [rusoto-rds/src/main.rs](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rds/src/main.rs) for the full code.  We'll be using [Rusoto](https://github.com/rusoto/rusoto/) 0.24.0, the latest release as of writing this post.

The meat of the program is this:

```rust
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
```

A lot to unravel.

The first thing we do is create an AWS credential object:

```rust
let credentials = DefaultCredentialsProvider::new().unwrap();
```

This creates a Rusoto credential chain.  It will source credentials according to [AWS best practices](https://github.com/rusoto/rusoto/blob/master/AWS-CREDENTIALS.md).

```rust
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
```

This code creates a Rusoto client for AWS RDS.  Then it makes a new `CreateDBInstanceMessage`, as specified by the AWS RDS API definition.  We set database storage/disk size, disable backups, use a `t2.micro` size and we set our username, password and database name, along with setting it to a single availability zone (AZ) since this is a non-production database.  We wrap it up by telling Rusoto to use defaults for the rest of the request.

Finally, we execute the request to create the RDS instance:

```rust
let db_creation_result = rds_client.create_db_instance(&create_db_instance_request).unwrap();
```

Since creating the database can take a few minutes, we poll AWS for its status:

```rust
let describe_instances_request = DescribeDBInstancesMessage {
    db_instance_identifier: Some(database_instance_name.to_string()),
    ..Default::default()
};
```

```rust
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
```

This code waits for the instance to become available by checking for the RDS instance by name.

```rust
let endpoint_address = endpoint.address.unwrap();
let endpoint_port = endpoint.port.unwrap();
println!("\n\nendpoint: {:?}", format!("{}:{}", endpoint_address, endpoint_port));
```

When the database is available, we extract the connection string and print it.  Since the DNS name AWS creates for the RDS instance is unique, we'll put that in the `.env` file in `rusoto-rocket`.

Example of `.env`, using `localhost` instead of the RDS DNS name:

`DATABASE_URL=postgres://masteruser:TotallySecurePassword501@localhost/rusoto_rocket`

Now, to create the database: in the `rusoto-rds` directory, run `cargo run` to create a new database and wait for it to be available. Populate `.env` DNS name the with the output of `rusoto-rds`, replacing `localhost` in this example.

## Security groups

Using the AWS Console, add a new rule to the security group the RDS instance is using.  Allow inbound traffic on port 5432
from your IP address.  If the following `diesel` commands time out, double check you can reach the instance.  A common gotcha is security groups blocking ingress.

## Diesel

We've already installed the Diesel CLI with `cargo install diesel_cli --features "postgres" --no-default-features`.

Ensure the `.env` file in `rusoto-rocket` has the connection string from the new RDS instance, including username and password:

`DATABASE_URL=postgres://postgres:TotallySecurePassword501@localhost/rusoto_rocket`

The up and down files have been populated in this sample.  They are available at https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rocket/migrations/20170503003554_hit_counter .

[Up file](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket/migrations/20170503003554_hit_counter/up.sql):
```SQL
CREATE TABLE hits (
    id SERIAL PRIMARY KEY,
    hits_so_far SERIAL NOT NULL
)
```

[Down file](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket/migrations/20170503003554_hit_counter/down.sql):
```SQL
DROP TABLE hits
```

The [schema file](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket/src/schema.rs) infers the schema via the database:

```rust
infer_schema!("dotenv:DATABASE_URL");
```

The ORM models we use are in the [models file](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket/src/models.rs):

```rust
use schema::hits;

#[derive(Queryable, Insertable, Debug)]
#[table_name="hits"]
pub struct Hit {
    pub id: i32,
    pub hits_so_far: i32,
}
```

We use the type `Hit` to describe our hit counter.  It `derive`s Queryable, Insertable and Debug, in the table `hits`.  There's an `id` field which we gloss over by using a constant and the `hits_so_far` field where we store the number of hits the site has seen.  Except `Debug`, the `derive` fields are used by Diesel.

In the `rusoto-rocket` directory, run `diesel setup`.  This will connect to RDS and create the database with the required schema.

If the command times out, ensure your security groups allow inbound access from your IP address.

## Making the Rocket site

This sample site follows the guide at https://rocket.rs/ .  Rocket is why nightly Rust is required: Rusoto and Diesel work on stable Rust.

The source code for the Rocket site is located [on Github](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rocket).

The [Cargo.toml](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket/Cargo.toml) brings in the following dependencies:

```toml
[dependencies]
rocket = "0.2.6"
rocket_codegen = "0.2.6"
diesel = { version = "0.11.0", features = ["postgres"] }
diesel_codegen = { version = "0.11.0", features = ["postgres"] }
dotenv = "0.8.0"
```

We're using Rocket 0.2.6 along with its codegen library, Diesel 0.11.0 with Postgres with its codegen library and dotenv for supplying configuration to Diesel.

In the `bin/main` directory we have the entirety of the Rocket site:

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rusoto_rocket;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::rusoto_rocket::*;
use self::rusoto_rocket::models::*;

#[get("/")]
fn index() -> String {
    use rusoto_rocket::schema::hits::dsl::*;
    // we should have connection made outside this handler:
    let connection = establish_connection();
    let hits_from_db = hits.filter(id.eq(1)).limit(1).load::<Hit>(&connection).expect("Couldn't load hits, yo.");
    // increment hits:
    let hits_weve_seen = hits_from_db.first().unwrap().hits_so_far;
    increment_hit(&connection, 1, hits_weve_seen + 1);
    format!("Hello, world!  Hits: {:?}", hits_weve_seen).to_string()
}

fn main() {
    let connection = establish_connection();
    create_hit(&connection, 1);
    rocket::ignite().mount("/", routes![index]).launch();
}

pub fn increment_hit(conn: &PgConnection, id: i32, new_hits: i32) {
    use schema::hits;
    use rusoto_rocket::schema::hits::dsl::hits as myhits;

    let result = diesel::update(myhits.find(id))
        .set(hits::hits_so_far.eq(new_hits))
        .execute(conn);

    match result {
        Ok(_) => (),
        Err(e) => println!("Couldn't update hit counter: {}", e),
    };
}
```

That's a lot to take in!  Let's break it down:

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rusoto_rocket;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::rusoto_rocket::*;
use self::rusoto_rocket::models::*;
```

We enable Rust plugins and the Rocket codegen plugin.  Then we bring in the required crates: `diesel` for database access, `dotenv` for configuration, `rocket` for the site and code for this walkthrough, `rusoto_rocket`.  We `use` the required modules: `diesel` boilerplate, Postgres libraries and our `rusoto_rocket` code and database models.

```rust
#[get("/")]
fn index() -> String {
    use rusoto_rocket::schema::hits::dsl::*;
    // we should have connection made outside this handler:
    let connection = establish_connection();
    let hits_from_db = hits.filter(id.eq(1)).limit(1).load::<Hit>(&connection).expect("Couldn't load hits, yo.");
    // increment hits:
    let hits_weve_seen = hits_from_db.first().unwrap().hits_so_far;
    increment_hit(&connection, 1, hits_weve_seen + 1);
    format!("Hello, world!  Hits: {:?}", hits_weve_seen).to_string()
}

fn main() {
    let connection = establish_connection();
    create_hit(&connection, 1);
    rocket::ignite().mount("/", routes![index]).launch();
}
```

Starting with `main`, we establish a connection to the database.  Then we create our first `hit` record with an `id` of `1`.  Then we call `rocket::ignite().mount("/", routes![index]).launch();` to bind our Rocket site to `/` with the `index` route as the only route.

The `index` route is defined by the `fn index()` function and the `#[get("/")]` tells Rocket to map it to the root URL: no path required for that endpoint.

```rust
pub fn increment_hit(conn: &PgConnection, id: i32, new_hits: i32) {
    use schema::hits;
    use rusoto_rocket::schema::hits::dsl::hits as myhits;

    let result = diesel::update(myhits.find(id))
        .set(hits::hits_so_far.eq(new_hits))
        .execute(conn);

    match result {
        Ok(_) => (),
        Err(e) => println!("Couldn't update hit counter: {}", e),
    };
}
```

Finally, our `increment_hit` function: this uses Diesel to update the database record.  We reference the required schema item, rename the Diesel DSL `hits` as `myhits` and run an update.  The actual integer increment happens in the `index` function.  More on this in the Diesel section.

## Connecting it all

Run `cargo run` in `rusoto-rocket` directory.  This will spin up a Rocket webserver on http://localhost:8000.
Visit that page to see the hit counter.  Refresh the page and see it increment by one for every page visit.
The data is stored in the RDS instance on AWS. 🎉

## Cleaning up

To ensure the database doesn't keep running and potentially run up AWS bills, log in to the AWS Console and delete the
RDS DB instance.

## Demo vs longer term infrastructure

As a demo, there's a lot of best practices ignored in favor of concise code.  An incomplete list of things that should be addresses when making a real service:

* Lots of `unwrap()` in this sample code.  Check for errors instead of that.
* Deploys of a real site should use [Cloudformation](https://aws.amazon.com/cloudformation/) via [Troposphere](https://github.com/cloudtools/troposphere) for infrastructure, such as the RDS instance.
* To deploy an actual Rocket site, use [CodeDeploy](https://aws.amazon.com/codedeploy/), [ElasticBeanstalk](https://aws.amazon.com/elasticbeanstalk/) or [Elastic Container Service](https://aws.amazon.com/ecs/) instead of copying files to an AWS EC2 instance.

## Rusoto homework

Instead of going through the AWS Console, we can modify the security groups to allow ingress from our IP address using Rusoto.  Find the database's security group and `allow` inbound traffic from your IP address.  Security groups are under [EC2 in Rusoto](https://rusoto.github.io/rusoto/rusoto_ec2/struct.CreateSecurityGroupRequest.html).
