use super::forms::{AddItemForm, ConvertItemForm, ItemOnShelfForm, TransferItemForm, UpdateItemForm};
use super::responses;
use crate::common::{AppData, ExtraRoomId, ExtraShelfId, ListingCriteria};
use crate::errors::ServiceError;
use crate::models::item::ItemId;
use crate::web::api::v1::extractors::bearer_token::Extract;
use crate::web::api::v1::responses::OkResponseData;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use std::sync::Arc;

#[allow(clippy::unused_async)]
pub async fn add_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<AddItemForm>,
) -> Response {
    if let Some(desc) = &item_form.description {
        return match app_data
            .item_service
            .add_item_with_desc(&item_form.name, &desc, &item_form.sn)
            .await
        {
            Ok(item_id) => responses::mutated_item(item_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    match app_data.item_service.add_item(&item_form.name, &item_form.sn).await {
        Ok(item_id) => responses::mutated_item(item_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn delete_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(item_id): Path<ItemId>,
) -> Response {
    match app_data.item_service.remove_item(&item_id).await {
        Ok(_) => responses::mutated_item(item_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn update_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(item_id): Path<ItemId>,
    Json(item_form): Json<AddItemForm>,
) -> Response {
    match app_data
        .item_service
        .update_item(&item_id, &item_form.name, &item_form.description, &item_form.sn)
        .await
    {
        Ok(_) => responses::mutated_item(item_id).into_response(),
        Err(error) => error.into_response(),
    }
}
#[allow(clippy::unused_async)]
pub async fn patch_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(item_id): Path<ItemId>,
    Json(item_form): Json<UpdateItemForm>,
) -> Response {
    if let Some(name) = &item_form.name {
        return match app_data.item_service.update_item_name(&item_id, &name).await {
            Ok(_) => responses::mutated_item(item_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    if let Some(desc) = &item_form.description {
        return match app_data.item_service.update_item_desc(&item_id, &desc).await {
            Ok(_) => responses::mutated_item(item_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    if let Some(sn) = &item_form.sn {
        return match app_data.item_service.update_item_sn(&item_id, sn).await {
            Ok(_) => responses::mutated_item(item_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    ServiceError::PayloadNotValid.into_response()
}

#[allow(clippy::unused_async)]
pub async fn get_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(item_id): Path<ItemId>,
) -> Response {
    match app_data.item_service.get_item(&item_id).await {
        Ok(item) => responses::get_item(item).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_all_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Query(criteria): Query<ListingCriteria>,
    Query(extra_shelf): Query<ExtraShelfId>,
    Query(extra_room): Query<ExtraRoomId>,
) -> Response {
    let spec = app_data.cfg.spec_from_criteria(&criteria).await;
    if let Some(shelf_id) = extra_shelf.shelf_id {
        return match app_data.item_service.get_items_on_shelf(&spec, shelf_id).await {
            Ok(items) => Json(OkResponseData { data: items }).into_response(),
            Err(error) => error.into_response(),
        };
    }
    if let Some(room_id) = extra_room.room_id {
        return match app_data.item_service.get_items_in_room(&spec, room_id).await {
            Ok(items) => Json(OkResponseData { data: items }).into_response(),
            Err(error) => error.into_response(),
        };
    }
    match app_data.item_service.get_items(&spec).await {
        Ok(items) => Json(OkResponseData { data: items }).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn transfer_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(item_form): Json<TransferItemForm>,
) -> Response {
    match app_data
        .item_service
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
    Path(item_id): Path<ItemId>,
    Json(item_form): Json<ItemOnShelfForm>,
) -> Response {
    match app_data
        .item_service
        .withdraw_item(&item_id, item_form.count, item_form.shelf_id)
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
    Path(item_id): Path<ItemId>,
    Json(item_form): Json<ItemOnShelfForm>,
) -> Response {
    match app_data
        .item_service
        .deposit_item(&item_id, item_form.count, item_form.shelf_id)
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
    match app_data.item_service.convert_item(item_form.from, item_form.into).await {
        Ok(_) => Json(OkResponseData { data: "todo" }).into_response(),
        Err(error) => error.into_response(),
    }
}
