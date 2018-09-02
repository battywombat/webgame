#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;
mod database;

use std::collections::HashMap;
use rocket::State;
use rocket_contrib::Template;
use database::DbConn;

#[get("/")]
fn index(_db_conn: State<DbConn>) -> Template {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("message", "Hello, World!");
    Template::render("index", map)
}

#[get("/character/<id>")]
fn get_character(db_conn: State<DbConn>, id: i32) -> Template {
    let conn = db_conn.lock().unwrap();
    let mut map = HashMap::new();
    let character = match database::get_character(&conn, id) {
        Ok(c) => c,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Template::render("404", map);
        }
        Err(e) => {
            map.insert("errormessage", format!("{:?}", e));
            return Template::render("500", map);
        }
    };
    println!("{:?}", character);
    map.insert("title", format!("{}'s Character Sheet", character.name));
    map.insert("name", character.name);
    map.insert("strength", character.strength.to_string());
    map.insert("magic", character.magic.to_string());
    map.insert("vitality", character.vitality.to_string());
    map.insert("agility", character.agility.to_string());
    map.insert("luck", character.luck.to_string());
    Template::render("character", map)
}

fn main() {
    let conn = database::create_connection_with_testdata(":memory:", "schema.sql", "testdata.sql").expect("Failed to open database");
    rocket::ignite().mount("/", routes![index, get_character])
                    .attach(Template::fairing())
                    .manage(conn)
                    .launch();
}