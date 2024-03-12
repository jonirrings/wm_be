-- Add migration script here
CREATE TABLE IF NOT EXISTS shelf
(
    shelf_id   BIGSERIAL PRIMARY KEY,
    name       TEXT        NOT NULL UNIQUE,
    layer      BIGINT      NOT NULL DEFAULT 0,
    room_id    BIGINT      NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    FOREIGN KEY (room_id) REFERENCES rooms (room_id)
);

CREATE TRIGGER shelf_trig
    BEFORE UPDATE
    ON shelf
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();