-- Add migration script here
CREATE TABLE IF NOT EXISTS user_profiles
(
    user_id        BIGINT      NOT NULL PRIMARY KEY,
    username       VARCHAR(50) NOT NULL UNIQUE,
    email          VARCHAR(50) UNIQUE,
    email_verified BOOL        NOT NULL DEFAULT FALSE,
    bio            TEXT,
    avatar         TEXT,
    updated_at     DATETIME ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);