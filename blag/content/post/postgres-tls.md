+++
date = "2018-09-13T00:28:53-07:00"
draft = false
title = "Postgres over TLS with postgres and r2d2_postgres"
+++

"Dance like nobody's watching. Encrypt like everybody is."

In this post we'll go over how to get the [postgres crate](https://crates.io/crates/postgres) and [r2d2_postgres](https://crates.io/crates/r2d2_postgres) working with [openssl](https://crates.io/crates/openssl) for connection pooling with TLS.

Source code is [available](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/postgres-tls-tester).

## Goal

Modern web applications are built with resiliency and fault tolerance in mind. For our example, we'll be examining parts of a web application backed by a Postgres database.

When services launch, their database won't always be available. This scenario shouldn't cause the service to crash. Instead, it should wait until its backing data store is accessible, then continue.

## Ping

The first step is to gather the details needed to start the service. This is usually configuration and credentials from environment variables in [12 Factor App](https://12factor.net/) fashion. After that, the service will ping the database until it's available.

```rust
// Usual web app startup things here: read config from environment variables,
// do any one-time startup work, etc...
let conn_string = "";
println!("Conn string: {}", conn_string);
let dbssl = "require"; // pretend we got this from environment variables

// First: ping the database until it accepts a connection:
loop {
    // We do connection building and other work inside the loop because we can't
    // .clone() ping_db_ssl_mode:
    let mut connbuilder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();

    // https://www.postgresql.org/docs/current/static/libpq-ssl.html describes the modes
    match dbssl.to_lowercase().as_ref() {
        "require" | "prefer" | "allow" => connbuilder.set_verify(postgres::tls::openssl::openssl::ssl::SSL_VERIFY_NONE),
        _ => (), // by default we verify certs: it's like either verify-ca or verify-full, TBD
    }

    let negotiator = postgres::tls::openssl::OpenSsl::from(connbuilder.build());
    let ping_db_ssl_mode = match dbssl.to_lowercase().as_ref() {
        "require" | "verify-ca" | "verify-full" => postgres::TlsMode::Require(&negotiator),
        // `prefer` and `allow` fall into here and will not try TLS. 
        // Not totally correct: please use at least `require` for real use.
        _ => postgres::TlsMode::None, 
    };

    match Connection::connect(conn_string.as_ref(), ping_db_ssl_mode) {
        Ok(_) => {
            println!("connected to db");
            break;
        },
        Err(e) => {
            println!("Couldn't connect to DB, sleeping a second. Error: {}", e);
            use std::{thread, time};
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}

println!("Postgres TLS tester, connected!");
// After we exit the loop, the connection we made should drop and disconnect.
```

As the comments point out, we look at what the `sslmode` settings are to determine how picky we are about encryption and certificates. For more details, review [the official Postgres docs on libpq-ssl](https://www.postgresql.org/docs/current/static/libpq-ssl.html).

Rephrased: if the connection specifies `require`, `prefer` or `allow`, we construct our `SslConnectorBuilder` to not verify the certificates the Postgres server returns. Later we use that if `require`, `verify-ca` or `verify-full` is specified. Otherwise we disable TLS when talking to the database.

## Connection pool

After we've successfully pinged the database, it's time to make our connection pool. Since making database connections are relatively expensive, we *really* want to keep a pool of them. The r2d2_postgres crate handles that for us, but we need to do a lot of the same work with configuring its TLS connections.

```rust
let mut builder = SslConnector::builder(::openssl::ssl::SslMethod::tls()).unwrap();
match dbssl.to_lowercase().as_ref() {
    "require" | "prefer" | "allow" => builder.set_verify(SslVerifyMode::empty()),
    _ => (), // by default we verify certs: it's like either verify-ca or verify-full, TBD
}

let negotiator = Box::new(::postgres::tls::openssl::OpenSsl::new().unwrap());
let db_ssl_mode = match dbssl.to_lowercase().as_ref() {
    "require" | "verify-ca" | "verify-full" => TlsMode::Require(negotiator),
    // `prefer` and `allow` fall into here and will not try TLS. 
    // Not totally correct: please use at least `require` for real use.
    _ => TlsMode::None, 
};

let manager = PostgresConnectionManager::new(conn_string.as_ref(), db_ssl_mode)
    .expect("Couldn't make postgres connection manager");
let pool = r2d2::Pool::new(manager).expect("Couldn't make pool from pg connection manager");
```

This is very similar, but has some tricky parts in it. Specifically, many `openssl` items are re-exported from the `postgres` crate and collide with ones from our `openssl` import but are not compatible. To get around that, we fully specify which structs and functions we want. We also `Box` our openssl connection to put it on the heap, since we'll be passing the connection pool into our actix-web handlers.

In my web app, I take the `pool` variable and put it into the actix-web server's application state for access in its handlers.

## Recap

Secure connections to the database are important but can be a little tricky to figure out in Rust. In this example we've used the `postgres` crate to connect to the database, supporting encryption while allowing plaintext if needed, and showed how to do the same with `r2d2_postgres`.

Go forth with your secure connections!
