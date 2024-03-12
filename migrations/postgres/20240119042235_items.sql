-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    item_id     BIGSERIAL PRIMARY KEY,
    name        TEXT        NOT NULL UNIQUE,
    sn          TEXT        NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ
);
CREATE TRIGGER items_trig
    BEFORE UPDATE
    ON items
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();