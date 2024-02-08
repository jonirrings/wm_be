-- Add migration script here
CREATE TABLE IF NOT EXISTS user_profiles
(
    user_id        INTEGER NOT NULL PRIMARY KEY,
    username       TEXT    NOT NULL UNIQUE,
    email          TEXT UNIQUE,
    email_verified BOOL    NOT NULL DEFAULT FALSE,
    bio            TEXT,
    avatar         TEXT,
    updated_at     DATETIME,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);

CREATE TRIGGER up_trig
    AFTER UPDATE
    ON user_profiles
BEGIN
    UPDATE user_profiles SET updated_at = datetime('now') WHERE user_id = NEW.user_id;
END;
