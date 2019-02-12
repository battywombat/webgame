extern crate rusqlite;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use rusqlite::Connection;
use rusqlite::Result;

#[database("sqlite_db")]
pub struct DbConn(rusqlite::Connection);

fn read_schema<P>(path: P) -> io::Result<String>
    where P: AsRef<Path> {
    let mut file = File::open(path)?;
    let mut schema = String::new();
    file.read_to_string(&mut schema)?;
    Ok(schema)
}

fn exec_file<P>(conn: &Connection, path: P) -> Result<()>
    where P: AsRef<Path> {
    let schema = match read_schema(path) {
        Ok(s) => s,
        Err(e) => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    };
    conn.execute_batch(schema.as_str())
}

fn create_connection_with_scripts<P>(database: P, schema: Option<P>, testdata: Option<P>) -> Result<()>
    where P: AsRef<Path> {
    let conn = Connection::open(database)?;
    if let Some(sc) = schema {
        exec_file(&conn, sc)?;
    }
    if let Some(data) = testdata {
        exec_file(&conn, data)?;
    }
    Ok(())
}

// pub fn create_connection<P>(database: P) -> Result<DbConn>
//     where P: AsRef<Path> {
//     create_connection_with_scripts(database, None, None)
// }

// pub fn create_connection_with_schema<P>(database: P, schema: P) -> Result<DbConn>
//     where P: AsRef<Path> {
//     create_connection_with_scripts(database, Some(schema), None)
// }

pub fn create_connection_with_testdata<P>(database: P, schema: P, testdata: P) -> Result<()>
    where P: AsRef<Path> {
    create_connection_with_scripts(database, Some(schema), Some(testdata))
}