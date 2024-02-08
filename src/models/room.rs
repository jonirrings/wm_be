use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::iso8601;
use time::OffsetDateTime;

pub type RoomId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Room {
    pub room_id: RoomId,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}
