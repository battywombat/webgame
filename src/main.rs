#![feature(plugin, custom_derive, decl_macro, never_type)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod database;
mod user;
mod character;
mod tiles;

use rocket::State;
use rocket::response::Redirect;
use rocket_contrib::Template;
use database::DbConn;

#[get("/")]
fn index(_db_conn: State<DbConn>) -> Redirect {
    Redirect::to("/login")
}


fn main() {
    let conn = database::create_connection_with_testdata(":memory:", "schema.sql", "testdata.sql").expect("Failed to open database");
    rocket::ignite().mount("/", routes![index,
                                        character::get_character_page,
                                        user::login, user::login_page, user::user_page, user::logout,
                                        tiles::get_tilemap, tiles::get_tile_file])
                    .attach(Template::fairing())
                    .manage(conn)
                    .launch();
}