extern crate rusqlite;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use rusqlite::Connection;
use rusqlite::Result;

pub type DbConn = Mutex<Connection>;

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

fn create_connection_with_scripts<P>(database: P, schema: Option<P>, testdata: Option<P>) -> Result<DbConn>
    where P: AsRef<Path> {
    let conn = Connection::open(database)?;
    if let Some(sc) = schema {
        exec_file(&conn, sc)?;
    }
    if let Some(data) = testdata {
        exec_file(&conn, data)?;
    }
    Ok(Mutex::new(conn))
}

// pub fn create_connection<P>(database: P) -> Result<DbConn>
//     where P: AsRef<Path> {
//     create_connection_with_scripts(database, None, None)
// }

// pub fn create_connection_with_schema<P>(database: P, schema: P) -> Result<DbConn>
//     where P: AsRef<Path> {
//     create_connection_with_scripts(database, Some(schema), None)
// }

pub fn create_connection_with_testdata<P>(database: P, schema: P, testdata: P) -> Result<DbConn>
    where P: AsRef<Path> {
    create_connection_with_scripts(database, Some(schema), Some(testdata))
}

#[derive(Debug)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub strength: u8,
    pub magic: u8,
    pub vitality: u8,
    pub agility: u8,
    pub luck: u8
}

pub fn get_character(conn: &Connection, id: i32) -> Result<Character> {
    let res = conn.query_row("SELECT id, cname, strength, magic, vitality, agility, luck FROM characters WHERE id=?", &[&id], |row| {
        let id = row.get_checked(0)?;
        let name = row.get_checked(1)?;
        let strength = row.get_checked(2)?;
        let magic = row.get_checked(3)?;
        let vitality = row.get_checked(4)?;
        let agility = row.get_checked(5)?;
        let luck = row.get_checked(6)?;
        Ok(Character {
            id,
            name,
            strength,
            magic,
            vitality,
            agility,
            luck
        })
    });

    match res {
        Ok(s) => s,
        Err(e) => Err(e)
    }
}