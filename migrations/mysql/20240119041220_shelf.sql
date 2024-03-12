-- Add migration script here
CREATE TABLE IF NOT EXISTS shelf
(
    shelf_id   BIGINT      NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name       VARCHAR(50) NOT NULL UNIQUE,
    layer      BIGINT      NOT NULL DEFAULT 0,
    room_id    BIGINT      NOT NULL,
    created_at DATETIME    NOT NULL DEFAULT current_timestamp,
    updated_at DATETIME ON UPDATE current_timestamp,
    FOREIGN KEY (room_id) REFERENCES rooms (room_id)
);