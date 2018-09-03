use std;
use rusqlite;

use std::collections::HashMap;
use std::fmt::{Formatter, Display};
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request, FlashMessage, Form};
use rocket::response::{Redirect, Flash};
use rocket::outcome::IntoOutcome;
use rocket_contrib::Template;
use rusqlite::Connection;
use database::DbConn;

#[derive(Debug, FromForm)]
pub struct User {
    pub username: String,
    pub password: String
}

pub struct UserId(i32);

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, !> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserId(id))
            .or_forward(())
    }
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("errormessage", msg.msg());
    } else {
        context.insert("errormessage", "");
    }

    Template::render("login", &context)
}

#[get("/user")]
pub fn user_page(db_conn: State<DbConn>, userid: UserId) -> Template {
    let mut map = HashMap::new();
    let user = get_user_by_id(&db_conn.lock().unwrap(), userid.0).unwrap();
    map.insert("title", String::from("User Page"));
    map.insert("name", user.username);
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

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie.named("user_id"));
    Redirect::to("/login")
}

#[derive(Debug)]
struct InvalidLoginError;

impl std::error::Error for InvalidLoginError {}

impl Display for InvalidLoginError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str("Invalid login info")
    }
}

fn validate(conn: &Connection, user: &User) -> Result<i32, Box<std::error::Error>> {
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

fn get_user_by_id(conn: &Connection, userid: i32) -> Result<User, rusqlite::Error> {
    conn.query_row("SELECT username, password FROM users where id=?", &[&userid], |row| {
        let username = row.get(0);
        let password = row.get(1);
        User{
            username,
            password
        }
    })
}