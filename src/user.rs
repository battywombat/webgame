use std;
use rusqlite;

use std::collections::HashMap;
use std::fmt::{Formatter, Display};
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request, FlashMessage, Form};
use rocket::response::{Redirect, Flash};
use rocket::Outcome;
use rocket_contrib::templates::Template;
use rusqlite::Connection;
use database::DbConn;

#[derive(Debug, FromForm)]
pub struct User {
    pub username: String,
    pub password: String
}

pub struct UserLogin {
    id: i32,
    character: Option<i32>
}

#[derive(Debug)]
pub struct AuthenticationError;

impl<'a, 'r> FromRequest<'a, 'r> for UserLogin {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserLogin, Self::Error> {
        let mut cookies = request.cookies();
        let id = match cookies.get_private("user_id").and_then(|cookie| cookie.value().parse().ok()) {
            Some(i) => i,
            None => return Outcome::Forward(())
        };
        let character = cookies.get_private("user_character").and_then(|cookie| cookie.value().parse().ok());
        Outcome::Success(UserLogin {
            id,
            character
        })
    }
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("errormessage", msg.msg());
    } else {
        context.insert("errormessage", "");
    }

    Template::render("login", &context)
}

#[get("/user")]
pub fn user_page(db_conn: State<DbConn>, user: Option<UserLogin>) -> Result<Template, Flash<Redirect>> {
    match user {
        Some(user) => {
            let mut map = HashMap::new();
            let conn = match db_conn.lock() {
                Ok(c) => c,
                Err(_) => return Err(Flash::error(Redirect::to("login"), "Something went wrong with our database."))
            };
            // Will never fail, and if it does, we have bigger problems.
            let user = get_user_by_id(&conn, user.id).unwrap();
            map.insert("title", String::from("User Page"));
            map.insert("name", user.username);
            Ok(Template::render("user", map))
        },
        None => {
            Err(Flash::error(Redirect::to("login"), "Please log in first."))
        }
    }
}

#[post("/login", data= "<user_form>")]
pub fn login(db_conn: State<DbConn>, mut cookies: Cookies, user_form: Form<User>) -> Result<Redirect, Flash<Redirect>> {
    let conn = db_conn.lock().unwrap();
    match validate(&conn, &user_form) {
        Ok(id) => {
            cookies.add_private(Cookie::new("user_id", id.to_string()));
            Ok(Redirect::to("user"))
        },
        Err(_) => {
            Err(Flash::error(Redirect::to("login"), "Invalid username/password"))
        }
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("login")
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