-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    user_id       BIGSERIAL PRIMARY KEY,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    administrator BOOL        NOT NULL DEFAULT FALSE
)