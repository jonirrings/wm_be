-- Add migration script here
CREATE TABLE IF NOT EXISTS stock
(
    stock_id   BIGSERIAL PRIMARY KEY,
    item_id    BIGINT      NOT NULL,
    shelf_id   BIGINT      NOT NULL,
    count      INTEGER     NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    FOREIGN KEY (item_id) REFERENCES items (item_id),
    FOREIGN KEY (shelf_id) REFERENCES shelf (shelf_id)
);

CREATE TRIGGER stock_trig
    BEFORE UPDATE
    ON stock
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();