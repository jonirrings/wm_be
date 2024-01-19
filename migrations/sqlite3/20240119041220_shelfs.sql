-- Add migration script here
CREATE TABLE IF NOT EXISTS shelf
(
    shelf_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name     TEXT    NOT NULL UNIQUE,
    layer    INTEGER NOT NULL DEFAULT 0,
    room_id  INTEGER NOT NULL,
    FOREIGN KEY (room_id) REFERENCES rooms (room_id)
)