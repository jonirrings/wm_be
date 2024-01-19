-- Add migration script here
CREATE TABLE IF NOT EXISTS shelf_items
(
    item_id  INTEGER NOT NULL,
    shelf_id INTEGER NOT NULL,
    count    INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (item_id),
    FOREIGN KEY (shelf_id) REFERENCES shelf (shelf_id)
)