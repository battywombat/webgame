extern crate rusqlite;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use self::rusqlite::Connection;
use self::rusqlite::Result;

pub type DbConn = Mutex<Connection>;

fn read_schema<P>(path: P) -> io::Result<String>
    where P: AsRef<Path> {
    let mut file = File::open(path)?;
    let mut schema = String::new();
    file.read_to_string(&mut schema)?;
    Ok(schema)
}

fn init_schema<P>(conn: &Connection, path: P) -> Result<()>
    where P: AsRef<Path> {
    let schema = match read_schema(path) {
        Ok(s) => s,
        Err(e) => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    };
    conn.execute_batch(schema.as_str())
}

pub fn create_connection<P1, P2>(database: P1, schema: P2) -> Result<DbConn>
    where P1: AsRef<Path>, P2: AsRef<Path> {
    let conn = Connection::open(database)?;
    init_schema(&conn, schema)?;
    Ok(Mutex::new(conn))
}