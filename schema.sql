DROP TABLE IF EXISTS characters;

CREATE TABLE characters(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cname VARCHAR(20),
    strength INTEGER,
    magic INTEGER,
    vitality INTEGER,
    agility INTEGER,
    luck INTEGER
);