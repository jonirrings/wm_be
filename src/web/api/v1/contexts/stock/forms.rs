use serde_derive::{Deserialize, Serialize};

use crate::models::item::{ItemId, ItemXShelf};
use crate::models::shelf::ShelfId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemOnShelfForm {
    pub item_id: ItemId,
    pub shelf_id: ShelfId,
    pub count: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferItemForm {
    pub item_id: ItemId,
    pub shelf_from: ShelfId,
    pub shelf_to: ShelfId,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertItemForm {
    pub from: Vec<ItemXShelf>,
    pub into: Vec<ItemXShelf>,
}
