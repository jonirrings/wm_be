-- Add migration script here
CREATE TABLE IF NOT EXISTS shelf
(
    shelf_id   INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    name       TEXT     NOT NULL UNIQUE,
    layer      INTEGER  NOT NULL DEFAULT 0,
    room_id    INTEGER  NOT NULL,
    created_at DATETIME NOT NULL DEFAULT current_timestamp,
    updated_at DATETIME,
    FOREIGN KEY (room_id) REFERENCES rooms (room_id)
);
CREATE TRIGGER shelf_trig
    AFTER UPDATE
    ON shelf
BEGIN
    UPDATE shelf SET updated_at = datetime('now') WHERE shelf_id = NEW.shelf_id;
END;