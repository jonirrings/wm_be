use axum::Json;
use crate::models::shelf::{Shelf, ShelfId};
use crate::web::api::v1::responses::OkResponseData;

pub fn mutated_shelf(shelf_id: ShelfId) -> Json<OkResponseData<ShelfId>> {
    Json(OkResponseData { data: shelf_id })
}

pub fn get_shelf(shelf: Shelf) -> Json<OkResponseData<Shelf>> {
    Json(OkResponseData { data: shelf })
}