-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    room_id     BIGINT      NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name        VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    created_at  DATETIME    NOT NULL DEFAULT current_timestamp,
    updated_at  DATETIME ON UPDATE current_timestamp
);