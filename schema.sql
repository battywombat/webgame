DROP TABLE IF EXISTS users;

CREATE TABLE users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(20),
    password CHAR(20)
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
     FOREIGN KEY(player) REFERENCES users(id)
);