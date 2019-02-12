#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
// extern crate rusqlite;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod database;
mod user;
mod character;
mod graphics;

use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::rusqlite;

use database::DbConn;

#[get("/")]
fn index(_db_conn: DbConn) -> Redirect {
    Redirect::to("/login")
}

fn main() {
    database::create_connection_with_testdata("db.sqlite", "schema.sql", "testdata.sql").expect("error with creation");
    rocket::ignite().mount("/", routes![index,
                                        character::get_character_page,
                                        user::login, user::login_page, user::user_page, user::logout,
                                        graphics::get_tile_file, graphics::get_tilemap])
                    .attach(DbConn::fairing())
                    .attach(Template::fairing())
                    .launch();
}