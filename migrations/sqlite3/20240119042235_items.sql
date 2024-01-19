-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    item_id     INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL UNIQUE,
    sn          TEXT    NOT NULL UNIQUE,
    description TEXT
)