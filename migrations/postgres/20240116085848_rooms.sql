-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    room_id     BIGSERIAL PRIMARY KEY,
    name        TEXT        NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ
);

CREATE TRIGGER rooms_trig
    BEFORE UPDATE
    ON rooms
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();