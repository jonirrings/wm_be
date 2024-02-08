-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    item_id     INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT     NOT NULL UNIQUE,
    sn          TEXT     NOT NULL UNIQUE,
    description TEXT,
    created_at  DATETIME NOT NULL DEFAULT current_timestamp,
    updated_at  DATETIME
);
CREATE TRIGGER items_trig
    AFTER UPDATE
    ON items
BEGIN
    UPDATE items SET updated_at = datetime('now') WHERE item_id = NEW.item_id;
END;