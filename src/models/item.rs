use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::iso8601;
use time::OffsetDateTime;

#[allow(clippy::module_name_repetitions)]
pub type ItemId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Item {
    pub item_id: ItemId,
    pub name: String,
    pub description: Option<String>,
    ///serial number
    pub sn: String,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct ItemCompact {
    pub item_id: ItemId,
    pub name: String,
    pub description: Option<String>,
    ///serial number
    pub sn: String,
    pub count: i64,
    pub shelf_id: ShelfId,
    pub room_id: RoomId,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct ItemXShelf {
    pub item_id: ItemId,
    pub shelf_id: ShelfId,
    pub count: i64,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct ItemOnShelf {
    pub item_id: ItemId,
    pub item_name: String,
    pub shelf_id: ShelfId,
    pub shelf_name: String,
    pub count: i64,
    pub sn: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct ItemInRoom {
    pub item_id: ItemId,
    pub item_name: String,
    pub room_id: RoomId,
    pub room_name: String,
    pub count: i64,
}
