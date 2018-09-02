#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod database;

use std::collections::HashMap;
use rocket::State;
use rocket_contrib::Template;

#[get("/")]
fn index(_db_conn: State<database::DbConn>) -> Template {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("message", "Hello, World!");
    Template::render("index", map)
}

fn main() {
    let conn = database::create_connection(":memory:", "schema.sql").expect("Failed to open database");
    rocket::ignite().mount("/", routes![index])
                    .attach(Template::fairing())
                    .manage(conn)
                    .launch();
}