-- Add migration script here
CREATE TABLE IF NOT EXISTS user_profiles
(
    user_id        BIGINT NOT NULL PRIMARY KEY,
    username       TEXT   NOT NULL UNIQUE,
    email          TEXT UNIQUE,
    email_verified BOOL   NOT NULL DEFAULT FALSE,
    bio            TEXT,
    avatar         TEXT,
    updated_at     TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);

CREATE TRIGGER up_trig
    BEFORE UPDATE
    ON user_profiles
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
