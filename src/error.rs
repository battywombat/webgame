use rocket::http::Status;
use rocket::response::{Flash, Redirect};

pub enum WebGameError {
    SqlError(rusqlite::Error)
}

impl std::convert::From<rusqlite::Error> for WebGameError {
    fn from(e: rusqlite::Error) -> Self {
        WebGameError::SqlError(e)
    }
}

impl std::convert::From<WebGameError> for Status {
    fn from(e: WebGameError) -> Self {
        match e {
            WebGameError::SqlError(sqlerror) => match sqlerror {
                rusqlite::Error::QueryReturnedNoRows => Status::new(404, "Not found"),
                _ => Status::new(500, "Database Error")
            }
        }
    }
}

impl std::convert::From<WebGameError> for Flash<Redirect> {
    fn from(_e: WebGameError) -> Self {
        Flash::error(Redirect::to("/"), "An error occurred during validation")
    }
}
