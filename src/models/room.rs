use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type RoomId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Room {
    pub room_id: RoomId,
    pub name: String,
    pub description: Option<String>,
}