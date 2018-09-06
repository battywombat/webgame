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
