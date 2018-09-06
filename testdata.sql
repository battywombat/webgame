INSERT INTO users VALUES(
    1,
    "paule",
    "pass"
);

INSERT INTO tile_files (ntiles, path) VALUES(
    140,
    "resources/tiles/PathAndObjects_0.png"
);

INSERT INTO tiles (tile_file, sub_id) VALUES (1, 0), (1, 1), (1, 2), (1, 3), (1, 4);

INSERT INTO sectors (sector_name, width, height) VALUES ("Test Map", 10, 10);

INSERT INTO characters (cname, strength, magic, vitality, agility, luck, player, logged_in, sector) VALUES(
    "Hitoshura",
    20,
    15,
    18,
    15,
    11,
    1,
    0,
    1
);