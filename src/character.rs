use rusqlite;

use std::collections::HashMap;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use rusqlite::Connection;
use database::DbConn;

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

pub fn get_character(conn: &Connection, id: i32) -> rusqlite::Result<Character> {
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

#[get("/character/<id>/page")]
pub fn get_character_page(db_conn: DbConn, id: i32) -> Result<Template, Status> {
    let mut map = HashMap::new();
    let character = match get_character(&db_conn, id) {
        Ok(c) => c,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err(Status::NotFound),
        Err(_) => return Err(Status::new(500, "Database error"))
    };
    map.insert("title", format!("{}'s Character Sheet", character.name));
    map.insert("name", character.name);
    map.insert("strength", character.strength.to_string());
    map.insert("magic", character.magic.to_string());
    map.insert("vitality", character.vitality.to_string());
    map.insert("agility", character.agility.to_string());
    map.insert("luck", character.luck.to_string());
    Ok(Template::render("character", map))
}