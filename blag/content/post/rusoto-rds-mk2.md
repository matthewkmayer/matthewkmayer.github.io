+++
date = "2017-05-23T21:28:53-07:00"
draft = false
title = "Rusoto RDS walkthrough, mk 2"
+++

Since the publication of [Rusoto RDS walkthrough](https://matthewkmayer.github.io/blag/public/post/rusoto-rds-walkthrough/), a new version of Rusoto has been released: [0.25.0](https://github.com/rusoto/rusoto/releases/tag/rusoto-v0.25.0).  This includes some breaking changes so let's work through those.  We'll also be cleaning up some of the rougher edges in the previous walkthrough.

<!--more-->

### rusoto-rds-mk2

The previous project's [source code is on github](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rds).  We'll be making a new project based off that one.  You can see the final product [in rusoto-rds-mk2 folder](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-rds-mk2).

### Cargo.toml changes required for Rusoto 0.25.0

Before:

```
[dependencies]
rusoto = {version = "0.24", features = ["rds"]}
```

After:

```
[dependencies]
rusoto_core = {version = "0.25.0"}
rusoto_rds = {version = "0.25.0"}
```

Note there's now two crates needed: `rusoto_core` and `rusoto_rds`.  This is due to Rusoto now creating a crate per AWS service.  We're calling this [crategen](https://github.com/rusoto/rusoto/pull/628).

### Code changes required for Rusoto 0.25.0

Before:

```rust
extern crate rusoto;

use rusoto::rds::{RdsClient, CreateDBInstanceMessage, DescribeDBInstancesMessage};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};
```

After:

```rust
extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_rds::{Rds, RdsClient, CreateDBInstanceMessage, DescribeDBInstancesMessage};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};
```

We're bringing in both `rusoto_core` and `rusoto_rds` crates.  We're also bringing in `rusoto_rds::Rds` which is a trait for the RDS client.

The only other change in the code is changing the RDS endpoint type.

Before:

```rust
let endpoint : rusoto::rds::Endpoint;
```

After:

```rust
let endpoint : rusoto_rds::Endpoint;
```

That's all we need to change for migrating to Rusoto 0.25.0.  Let's move on to cleaning up other parts of the code.

### Making unwrap behavior slightly nicer

In rusoto-rds-mk2: we use `.expect()` instead of `.unwrap()`.  This doesn't prevent panics, but it does give us more information as to *why* things didn't work as expected.  Both [Result](https://doc.rust-lang.org/std/result/enum.Result.html) and [Option](https://doc.rust-lang.org/std/option/enum.Option.html) implement this.

Before:

```rust
let credentials = DefaultCredentialsProvider::new()
    .unwrap();
```

After:

```rust
let credentials = DefaultCredentialsProvider::new()
    .expect("Couldn't create AWS credentials provider.");
```

Knowing when to use `expect` instead of matching against Result or Option is worth understanding.  In our sample code, panicking if we can't get AWS credentials is probably what we want to do.  But what about calls to AWS?

```rust
let db_creation_result = rds_client.create_db_instance(&create_db_instance_request)
        .expect("Error sending create DB instance request");
```

[create_db_instance](http://rusoto.github.io/rusoto/rusoto_rds/trait.Rds.html#tymethod.create_db_instance) returns a type of `Result<CreateDBInstanceResult, CreateDBInstanceError>`.  If we use `expect` we throw away the `CreateDBInstanceError` which would allow us to see why Rusoto or AWS couldn't fulfill our request.  If we get this error result, let's print out what it says for debugging:

```rust
let db_creation_result = match rds_client.create_db_instance(&create_db_instance_request) {
    Ok(db_create_result) => db_create_result,
    Err(e) => {
        println!("Error making database instance: {}", e);
        panic!("No go on database creation.");
    },
};
```

This is better: the happy path set the `db_creation_result` variable with the result.  The unhappy path still panics, but we now have information on *why* it happened.  If we don't want to panic, we can modify this more:

```rust
let db_call_result = rds_client.create_db_instance(&create_db_instance_request);
if db_call_result.is_err() {
    // This `unwrap` on the `err()` call will show us the error we know is there:
    println!("Didn't successfully make the DB instance.  Error: {}", db_call_result.err().unwrap());
    // Since it didn't succeed, we can look at the error and see if we should retry or not.
    // For our sample, we'll panic.
    panic!("Error making DB instance creation request.");
}
```

Yes, we're still panicking in this example, but it shows we could do something else such as retrying the request.  We can look at the error and determine if it's a transient error and should be retried or if we should stop trying.

### Nightly compiler reminder

The rest of this post requires using a nightly version of Rust for the Rocket site.  This walkthrough uses `rustc 1.18.0-nightly (036983201 2017-04-26)`.  To switch to that nightly release, run `rustup default nightly-2017-04-26`.  The output of that command should look like this:

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

Now we're ready to play with the Rocket site some more!

### Diesel cleanup

Since the first iteration of this project was my first use of Diesel, I didn't know how to do an update and fetch the new value in one database call.  Thanks to rabidferret for [pointing that out on Reddit](https://www.reddit.com/r/rust/comments/6c7xpp/walkthrough_rocket_diesel_and_a_postgres_database/dhv2fgy/).  This is briefly covered in [the Diesel getting started page](http://diesel.rs/guides/getting-started/).  Instead of using `.execute` we'll use `.get_result`.  Per the Diesel docs, this `adds RETURNING * to the end of the query`.  Excellent!  Less database roundtrips is better.

First we'll add `numeric_expr!(hits::hits_so_far);` to the schema.rs file:

```rust
infer_schema!("dotenv:DATABASE_URL");
numeric_expr!(hits::hits_so_far);
```

This uses the [numeric_expr macro](http://docs.diesel.rs/diesel/macro.numeric_expr.html) to allow numeric operators.  We'll use the `+` operator to increment.

Back in `main.rs` for `rusoto-rocket-mk2` we'll make some changes.  We'll start with `increment_hits`.

Before:

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

This does the increment of the value.  We'll want to do the increment and return of the new value, like so:

```rust
pub fn increment_hit(conn: &PgConnection, hit_id: i32) -> Result<i32, diesel::result::Error> {
    use schema::hits;
    use schema::hits::dsl::*;
    use rusoto_rocket_mk2::schema::hits::dsl::hits as myhits;

    let result = diesel::update(myhits.find(hit_id))
        .set(hits::hits_so_far.eq(hits_so_far + 1))
        .get_result::<Hit>(conn);

    match result {
        Ok(hit_count) => Ok(hit_count.hits_so_far as i32),
        Err(e) => {
            println!("Couldn't update hit counter: {}", e);
            Err(e)
        }
    }
}
```

The main changes are using `get_result` instead of `execute` and setting the `hits_so_far` field to `hits_so_far + 1`.  This sets the incremented value and returns the updated value from the database.  We also change the function signature to return a `Result<i32, diesel::result::Error>`.  Before, it was a fire and forget, but now we use that for reporting the hits so far.

In the index function, we move from this:

```rust
#[get("/")]
fn index(db_conn: State<DbConn>) -> String {
    use rusoto_rocket::schema::hits::dsl::*;
    let my_db_conn = db_conn.inner().lock().expect("Couldn't get mutex lock on db connection");
    let hits_from_db = hits.filter(id.eq(1))
        .limit(1)
        .load::<Hit>(&my_db_conn as &PgConnection) // Explicit cast needed
        .expect("Couldn't load hits, yo.");
    // increment hits:
    let hits_weve_seen = hits_from_db.first().unwrap().hits_so_far;
    increment_hit(&my_db_conn, 1, hits_weve_seen + 1);
    format!("Hello, world!  Hits: {:?}", hits_weve_seen).to_string()
}
```

to this:

```rust
#[get("/")]
fn index(db_conn: State<DbConn>) -> String {
    let my_db_conn = db_conn.inner().lock().expect("Couldn't get mutex lock on db connection");
    let hits_from_db: i32 = match increment_hit(&my_db_conn, 1) {
        Ok(hits_after_increment) => hits_after_increment,
        Err(e) => {
            println!("Couldn't get new hit count from db: {}", e);
            -1
        },
    };
    format!("Hello, world!  Hits: {:?}", hits_from_db).to_string()
}
```

We've removed the database call to get the current state and instead match on the returned `Result` from `increment_hit`.  As a bonus, this fixes the off-by-one error in the original example where we'd see the page report 0 hits on our first visit.  The error branch of the `match` exposes the error state by reporting -1 hits if something goes wrong.

### Doing local testing with Docker instead of RDS

To play with the Diesel and Rocket portions of this walkthrough, we can use Docker to spin up a local Postgres container.  Run this command to create a Postgres container from the Alpine Linux distro:

```bash
docker run --name postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:alpine
```

We'll also need to adjust our `.env` file in [rusoto-rocket-mk2](https://github.com/matthewkmayer/matthewkmayer.github.io/blob/master/samples/rusoto-rocket-mk2/.env).  It should look like this:

```
DATABASE_URL=postgres://postgres:mysecretpassword@localhost/rusoto_rocket
```

Note the user is now `postgres` and the password matched what we supplied the `docker` command earlier.  We're using `localhost` since the Postgres image is listening on `localhost:5432`.

In the rusoto-rocket-mk2 directory, with the updated .env file, run `diesel setup`.  You'll see this success message:

```
Creating database: rusoto_rocket
Running migration 20170503003554
```

Now we can run `cargo run` to launch our Rocket site, using our local Docker image.  After it's running, visit [http://localhost:8000/](http://localhost:8000/) to see the hit counter.  Refresh and see the hits go up.  This time they are being stored in the local Postgres Docker container.

### Cleanup

`docker kill postgres` will kill the running container.  Verify it's no longer running by running `docker ps` and verify it's no longer listed.

### Diesel homework

If we start the Rocket site multiple times, we'll see an error like this:

```
thread 'main' panicked at 'Error saving new hit: DatabaseError(UniqueViolation, "duplicate key value violates unique constraint \"hits_pkey\"")', src/libcore/result.rs:859
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This is from calling `create_hit(&connection, 1);` in `main`.  In the lib.rs function, we blindly try to insert an item into the database.  Our migration file that creates the table says the `id` column in the `hits` table is a primary key, so we can't have duplicates and our application panics.  We can modify that to check if the entry exists and don't attempt an insert in that case.

As a workaround, we can run `diesel migration redo` to revert the database to a state where table are created but aren't populated, then can run `cargo run` in the Rocket site to start up again.