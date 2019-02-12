use std::fs::File;
use rusqlite;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rusqlite::Connection;

use database::DbConn;

#[derive(Debug, Serialize, Deserialize)]
struct TileFiles {
    tiles: Vec<(i32, String)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileRecord {
    id: i32,
    file_id: i32,
    sub_id: i32
}

#[get("/tilemap")]
pub fn get_tilemap(db_conn: DbConn) -> Result<Json<Vec<TileRecord>>, Status> {
    match get_all_tiles(&db_conn) {
        Ok(tiles) => Ok(Json(tiles)),
        Err(_) => Err(Status::new(500, "Error accessing database"))
    }
}

#[get("/tile/<id>")]
pub fn get_tile_file(db_conn: DbConn, id: i32) -> Result<File, Status> {
    let fp = match get_tile_path_from_id(&db_conn, id) {
        Ok(f) => f,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err(Status::NotFound),
        Err(_) => return Err(Status::new(500, "Error in database"))
    };
    match File::open(fp) {
        Ok(f) => Ok(f),
        Err(_) => Err(Status::NotFound)
    }
}

fn get_all_tiles(conn: &Connection) -> rusqlite::Result<Vec<TileRecord>> {
    let mut tiles = vec![];
    let mut stmt = conn.prepare_cached("SELECT id, tile_file, sub_id FROM tiles ORDER BY id")?;
    let tile_results = stmt.query_map(&[], |row| {
        TileRecord {
            id: row.get(0),
            file_id: row.get(1),
            sub_id: row.get(2)
        }
    })?;

    for tile in tile_results {
        tiles.push(tile?);
    }

    Ok(tiles)
}

fn get_tile_path_from_id(conn: &Connection, id: i32) -> rusqlite::Result<String> {
    let path = conn.query_row("SELECT path FROM tile_files WHERE id=?", &[&id], |row| {
        row.get(0)
    })?;
    Ok(path)
}
