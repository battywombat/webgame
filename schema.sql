DROP TABLE IF EXISTS users;

CREATE TABLE users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(20),
    password CHAR(20)
);

DROP TABLE IF EXISTS tile_files;
CREATE TABLE tile_files(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ntiles INTEGER,
    path TEXT
);

DROP TABLE IF EXISTS tiles;
CREATE TABLE tiles(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tile_file INTEGER,
    sub_id INTEGER,
    FOREIGN KEY (tile_file) REFERENCES tile_files(id)
);

DROP TABLE IF EXISTS sectors;
CREATE TABLE sectors(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sector_name CHAR(30),
    width INTEGER,
    height INTEGER
);

DROP TABLE IF EXISTS sectordata;
CREATE TABLE sectordata(
    sector INTEGER,
    x INTEGER,
    y INTEGER,
    tile INTEGER,
    PRIMARY KEY(sector, x, y),
    FOREIGN KEY(sector) REFERENCES sectors(id),
    FOREIGN KEY(tile) REFERENCES tiles(id)
);

DROP TABLE IF EXISTS characters;
CREATE TABLE characters(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cname VARCHAR(20),
    strength INTEGER,
    magic INTEGER,
    vitality INTEGER,
    agility INTEGER,
    luck INTEGER,
    player INTEGER,
    logged_in INTEGER,
    FOREIGN KEY(player) REFERENCES users(id)
);
