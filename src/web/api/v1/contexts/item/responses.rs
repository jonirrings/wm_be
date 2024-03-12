use axum::Json;

use crate::models::item::{Item, ItemId};
use crate::web::api::v1::responses::OkResponseData;

pub fn mutated_item(item_id: ItemId) -> Json<OkResponseData<ItemId>> {
    Json(OkResponseData { data: item_id })
}

pub fn get_item(item: Item) -> Json<OkResponseData<Item>> {
    Json(OkResponseData { data: item })
}
