-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    user_id       INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_at    TEXT    NOT NULL,
    administrator BOOL    NOT NULL DEFAULT FALSE
)