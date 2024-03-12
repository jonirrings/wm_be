-- Add migration script here
CREATE TABLE IF NOT EXISTS user_authentication
(
    user_id       BIGINT NOT NULL PRIMARY KEY,
    password_hash TEXT   NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
)