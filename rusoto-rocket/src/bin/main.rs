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