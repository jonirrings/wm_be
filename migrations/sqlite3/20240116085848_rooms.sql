-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    room_id     INTEGER NOT NULL PRIMARY KEY,
    name        TEXT    NOT NULL UNIQUE,
    description TEXT
)