#![feature(plugin, custom_derive, decl_macro, never_type)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate rusqlite;
mod database;
mod user;

use std::collections::HashMap;
use rocket::State;
use rocket::response::Redirect;
use rocket_contrib::Template;
use database::DbConn;

#[get("/")]
fn index(_db_conn: State<DbConn>) -> Redirect {
    Redirect::to("/login")
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
    rocket::ignite().mount("/", routes![index, get_character, user::login, user::login_page, user::user_page, user::logout])
                    .attach(Template::fairing())
                    .manage(conn)
                    .launch();
}