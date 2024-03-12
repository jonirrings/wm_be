-- Add migration script here
CREATE TABLE IF NOT EXISTS stock
(
    stock_id   BIGINT   NOT NULL PRIMARY KEY AUTO_INCREMENT,
    item_id    BIGINT   NOT NULL,
    shelf_id   BIGINT   NOT NULL,
    count      INTEGER  NOT NULL,
    created_at DATETIME NOT NULL DEFAULT current_timestamp,
    updated_at DATETIME ON UPDATE current_timestamp,
    FOREIGN KEY (item_id) REFERENCES items (item_id),
    FOREIGN KEY (shelf_id) REFERENCES shelf (shelf_id)
);