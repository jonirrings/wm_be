-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    user_id       BIGINT   NOT NULL PRIMARY KEY AUTO_INCREMENT,
    created_at    DATETIME NOT NULL DEFAULT current_timestamp,
    administrator BOOL     NOT NULL DEFAULT FALSE
)