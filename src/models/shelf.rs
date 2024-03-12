use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::iso8601;
use time::OffsetDateTime;

use super::room::RoomId;

pub type ShelfId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Shelf {
    pub shelf_id: ShelfId,
    pub name: String,
    pub layer: i64,
    pub room_id: RoomId,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}
