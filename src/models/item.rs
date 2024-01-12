use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(clippy::module_name_repetitions)]
pub type ItemId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Item {
    pub item_id: ItemId,
    pub name: String,
    pub description: Option<String>,
    pub sn: String,
}