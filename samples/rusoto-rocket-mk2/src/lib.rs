#![feature(plugin)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::Hit;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_hit<'a>(conn: &PgConnection, id: i32) -> Hit {
    use schema::hits;
    let new_hit = Hit {
        id: id,
        hits_so_far: 0,
    };

    diesel::insert(&new_hit).into(hits::table)
        .get_result(conn)
        .expect("Error saving new hit")
}
