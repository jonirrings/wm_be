use super::forms::{ConvertItemForm, ItemOnShelfForm, TransferItemForm};
use crate::common::{AppData, ListingCriteria};
use crate::models::room::RoomId;
use crate::models::shelf::ShelfId;
use crate::web::api::v1::extractors::bearer_token::Extract;
use crate::web::api::v1::responses::OkResponseData;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use std::sync::Arc;

#[allow(clippy::unused_async)]
pub async fn get_items_on_shelves_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Query(criteria): Query<ListingCriteria>,
) -> Response {
    todo!("todo")
}
#[allow(clippy::unused_async)]
pub async fn get_items_on_shelf_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(shelf_id): Path<ShelfId>,
    Query(criteria): Query<ListingCriteria>,
) -> Response {
    todo!("todo")
}
#[allow(clippy::unused_async)]
pub async fn get_items_in_rooms_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Query(criteria): Query<ListingCriteria>,
) -> Response {
    todo!("todo")
}
#[allow(clippy::unused_async)]
pub async fn get_items_in_room_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(room_id): Path<RoomId>,
    Query(criteria): Query<ListingCriteria>,
) -> Response {
    todo!("todo")
}
#[allow(clippy::unused_async)]
pub async fn transfer_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<TransferItemForm>,
) -> Response {
    match app_data
        .stock_service
        .transfer_item(&item_form.item_id, item_form.count, item_form.shelf_from, item_form.shelf_to)
        .await
    {
        Ok(_) => Json(OkResponseData { data: "todo" }).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn withdraw_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<ItemOnShelfForm>,
) -> Response {
    match app_data
        .stock_service
        .withdraw_item(&item_form.item_id, item_form.count, item_form.shelf_id)
        .await
    {
        Ok(_) => Json(OkResponseData { data: "todo" }).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn deposit_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<ItemOnShelfForm>,
) -> Response {
    match app_data
        .stock_service
        .deposit_item(&item_form.item_id, item_form.count, item_form.shelf_id)
        .await
    {
        Ok(_) => Json(OkResponseData { data: "todo" }).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn convert_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<ConvertItemForm>,
) -> Response {
    match app_data.stock_service.convert_item(item_form.from, item_form.into).await {
        Ok(_) => Json(OkResponseData { data: "todo" }).into_response(),
        Err(error) => error.into_response(),
    }
}
