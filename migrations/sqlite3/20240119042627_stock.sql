-- Add migration script here
CREATE TABLE IF NOT EXISTS stock
(
    stock_id   INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    item_id    INTEGER  NOT NULL,
    shelf_id   INTEGER  NOT NULL,
    count      INTEGER  NOT NULL,
    created_at DATETIME NOT NULL DEFAULT current_timestamp,
    updated_at DATETIME,
    FOREIGN KEY (item_id) REFERENCES items (item_id),
    FOREIGN KEY (shelf_id) REFERENCES shelf (shelf_id)
);

CREATE TRIGGER stock_trig
    AFTER UPDATE
    ON stock
BEGIN
    UPDATE stock SET updated_at = datetime('now') WHERE stock_id = NEW.stock_id;
END;