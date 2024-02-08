-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    user_id       INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_at    DATETIME NOT NULL DEFAULT current_timestamp,
    administrator BOOL     NOT NULL DEFAULT FALSE
)