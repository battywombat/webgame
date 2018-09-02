use std;

use std::collections::HashMap;
use std::fmt::{Formatter, Display};
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{Redirect, Flash};
use rocket_contrib::Template;
use rusqlite::Connection;
use database::DbConn;

#[derive(Debug, FromForm)]
pub struct User {
    pub username: String,
    pub password: String
}

#[get("/login")]
pub fn login_page(_db_conn: State<DbConn>) -> Template {
    let mut map = HashMap::new();
    map.insert("title", "Login Page");
    Template::render("login", map)
}

#[get("/user")]
pub fn user_page(db_conn: State<DbConn>) -> Template {
    let mut map = HashMap::new();
    map.insert("title", "User Page");
    Template::render("user", map)
}

#[post("/login", data= "<user_form>")]
pub fn login(db_conn: State<DbConn>, mut cookies: Cookies, user_form: Form<User>) -> Result<Redirect, Flash<Redirect>> {
    let user = user_form.get();
    let conn = db_conn.lock().unwrap();
    match validate(&conn, user) {
        Ok(id) => {
            cookies.add_private(Cookie::new("user_id", id.to_string()));
            Ok(Redirect::to("/user"))
        },
        Err(_) => {
            Err(Flash::error(Redirect::to("/login"), "Invalid username/password"))
        }
    }
}

#[derive(Debug)]
struct InvalidLoginError;

impl std::error::Error for InvalidLoginError {}

impl Display for InvalidLoginError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str("Invalid login info")
    }
}

pub fn validate(conn: &Connection, user: &User) -> Result<i32, Box<std::error::Error>> {
    let (id, db_user) = conn.query_row("SELECT id, username, password FROM users WHERE username=?", &[&user.username], |row| {
        let id = row.get(0);
        let username = row.get(1);
        let password = row.get(2);
        (id, User{
            username,
            password
        })
    })?;
    match user.password == db_user.password {
        true => Ok(id),
        false => Err(Box::new(InvalidLoginError{}))
    }
}