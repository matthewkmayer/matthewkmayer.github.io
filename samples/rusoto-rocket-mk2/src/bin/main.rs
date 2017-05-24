#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rusoto_rocket_mk2;

use std::sync::Mutex;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::State;
use self::rusoto_rocket_mk2::*;
use self::rusoto_rocket_mk2::models::*;

type DbConn = Mutex<PgConnection>;

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

fn main() {
    let connection = establish_connection();
    create_hit(&connection, 1);

    rocket::ignite()
        .manage(Mutex::new(connection))
        .mount("/", routes![index])
        .launch();
}

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