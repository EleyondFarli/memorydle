CREATE TABLE IF NOT EXISTS countries (
    country_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name varchar(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS puzzles(
    puzzle_id INTEGER PRIMARY KEY AUTOINCREMENT,
    puzzle_date DATE NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS images(
    image_id INTEGER PRIMARY KEY AUTOINCREMENT,
    puzzle_id INTEGER NOT NULL REFERENCES puzzles(puzzle_id),
    image_path TEXT NOT NULL,
    display_order INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user (
    used_uuid TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW())
);
