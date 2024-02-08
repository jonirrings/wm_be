-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    room_id     INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT     NOT NULL UNIQUE,
    description TEXT,
    created_at  DATETIME NOT NULL DEFAULT current_timestamp,
    updated_at  DATETIME
);

CREATE TRIGGER rooms_trig
    AFTER UPDATE
    ON rooms
BEGIN
    UPDATE rooms SET updated_at = datetime('now') WHERE room_id = NEW.room_id;
END;
