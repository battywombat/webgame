#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
mod database;

use std::collections::HashMap;
use rocket::State;
use rocket_contrib::Template::DbConn;
use database

#[get("/")]
fn index(_db_conn: State<DbConn>) -> Template {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("message", "Hello, World!");
    Template::render("index", map)
}

#[get("/character/<id>")]
fn get_character(db_conn: State<DbConn>, id: i32) -> Template {
    let mut map = HashMap::new();
}

fn main() {
    let conn = database::create_connection_with_testdata(":memory:", "schema.sql", "testdata.sql").expect("Failed to open database");
    rocket::ignite().mount("/", routes![index])
                    .attach(Template::fairing())
                    .manage(conn)
                    .launch();
}