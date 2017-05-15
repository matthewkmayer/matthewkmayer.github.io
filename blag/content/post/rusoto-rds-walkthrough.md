+++
date = "2017-05-14T21:28:53-07:00"
draft = false
title = "Rusoto RDS walkthrough"
+++

## Overview

Let's tie some great Rust crates together!  In this walkthrough, we'll use [Rusoto](https://github.com/rusoto/rusoto) to create a Postgres RDS database instance,
[Rocket.rs](https://github.com/SergioBenitez/Rocket) to make a web server and [Diesel](https://github.com/diesel-rs/diesel) to talk to the database on AWS to make a proof of concept hit counter.

<!--more-->

## Prerequisites

Default VPC (for publically accessibly RDS instances), AWS access keys in some fashion, Rust nightly.

Switch to Rust nightly:  `rustup default nightly`.

## Rocket site

Follows the guide at https://rocket.rs/ .  Rocket is why nightly Rust is required: Rusoto and Diesel work on stable Rust.

## Creating a Postgres RDS instance

See [rusoto-rds/src/main.rs](rusoto-rds/src/main.rs) for the full code.

In the `rusoto-rds` directory, run `cargo run` to create a new database and wait for it to be available.
Once the endpoint is available, put that in the `.env` file in `rusoto-rocket`.

Example:

`DATABASE_URL=postgres://masteruser:TotallySecurePassword501@localhost/rusoto_rocket`

The code will:

* Create a new database
* Wait for it to be created
* Extract the endpoint to use with Diesel

## Security groups
Using the AWS Console, add a new rule to the security group the RDS instance is using.  Allow inbound traffic on port 5432
from your IP address.

## Diesel basics
Install with `cargo install diesel_cli --features "postgres" --no-default-features`.

Set up `.env` to have the connection string from the new RDS instance, including username and password:

`DATABASE_URL=postgres://postgres:TotallySecurePassword501@localhost/rusoto_rocket`

The up and down files have been populated in this sample.

Up file:
```SQL
CREATE TABLE hits (
    id SERIAL PRIMARY KEY,
    hits_so_far SERIAL NOT NULL
)
```

Down file:
```SQL
DROP TABLE hits
```

In the `rusoto-rocket` directory, run `diesel setup`.  This will connect to RDS and create the database with the required schema.

If the command times out, ensure your security groups allow inbound access from your IP address.

## Connecting it all

Run `cargo run` in `rusoto-rocket` directory.  This will spin up a Rocket webserver on http://localhost:8000.
Visit that page to see the hit counter.  Refresh the page and see it increment by one for every page visit.
The data is stored in the RDS instance on AWS. 🎉

## Cleaning up

To ensure the database doesn't keep running and potentially run up AWS bills, log in to the AWS Console and delete the
RDS DB instance.

## Demo vs longer term infrastructure

* Use Cloudformation via Troposphere.
* Deploy via CodeDeploy, ElasticBeanstalk, Elastic Container Service instead of SCP.

Testing notes:
`docker run --name postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:alpine`
