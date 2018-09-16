extern crate env_logger;
extern crate openssl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use openssl::ssl::*;
use postgres::Connection;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use postgres::tls::openssl::openssl::ssl::{SslConnectorBuilder, SslMethod};

fn main() {
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

    // Now we create our r2d2 connection pool, which is similar:
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

    // In my actix-web appl, I put the pool into the application state so it can be shared:
    // app_state.db_pool = Some(pool);

    // And now the actix-web server is launched, where it has pooled database connections
    // through r2d2_postgres.
}
