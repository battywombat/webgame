use std::fs::File;
use rusqlite;
use rocket_contrib::Json;
use rocket::State;
use rusqlite::Connection;

use database::DbConn;

#[derive(Debug, Serialize, Deserialize)]
struct TileMap {
    tiles: Vec<(i32, String)>
}

#[get("/tilemap")]
fn get_tilemap(db_conn: State<DbConn>) -> Json<TileMap> {
    Json(get_all_tiles(&db_conn.lock().unwrap()).unwrap())
}

#[get("/tile/<id>")]
fn get_tile_file(db_conn: State<DbConn>, id: i32) -> File {
    let conn = &db_conn.lock().unwrap();
    File::open(get_tile_path_from_id(conn, id).unwrap()).unwrap()
}

fn get_all_tiles(conn: &Connection) -> rusqlite::Result<TileMap> {
    let mut tiles = vec![];
    let mut stmt = conn.prepare_cached("SELECT id, path FROM tile_files ORDER BY id")?;
                
    for tile in stmt.query_map(&[], |row| (row.get(0), row.get(1)))? {
        tiles.push(tile?);
    }
    Ok(TileMap {
        tiles
    })
}

fn get_tile_path_from_id(conn: &Connection, id: i32) -> rusqlite::Result<String> {
    let path = conn.query_row("SELECT path FROM tile_files WHERE id=?", &[&id], |row| {
        row.get(0)
    })?;
    Ok(path)
}
