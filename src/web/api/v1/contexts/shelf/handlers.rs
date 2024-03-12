use std::sync::Arc;

use axum::extract::{Extension, Path, Query};
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::common::{AppData, ExtraRoomId};
use crate::common::{ListingCriteria, PagedConf};
use crate::errors::ServiceError;
use crate::models::shelf::ShelfId;
use crate::web::api::v1::extractors::bearer_token::Extract;
use crate::web::api::v1::responses::OkResponseData;

use super::forms::{AddShelfForm, UpdateShelfForm};
use super::responses;

#[allow(clippy::unused_async)]
pub async fn add_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(shelf_form): Json<AddShelfForm>,
) -> Response {
    match app_data
        .shelf_service
        .add_shelf(&shelf_form.name, shelf_form.layer, shelf_form.room_id)
        .await
    {
        Ok(shelf_id) => responses::mutated_shelf(shelf_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn delete_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(shelf_id): Path<ShelfId>,
) -> Response {
    match app_data.shelf_service.remove_shelf(&shelf_id).await {
        Ok(_) => responses::mutated_shelf(shelf_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn batch_delete_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(ids): Json<Vec<i64>>,
) -> Response {
    match app_data.shelf_service.remove_shelves(&ids).await {
        Ok(res) => Json(res).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn update_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(shelf_id): Path<ShelfId>,
    Json(shelf_form): Json<AddShelfForm>,
) -> Response {
    match app_data
        .shelf_service
        .update_shelf(&shelf_id, &shelf_form.name, shelf_form.layer, shelf_form.room_id)
        .await
    {
        Ok(_) => responses::mutated_shelf(shelf_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn patch_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(shelf_id): Path<ShelfId>,
    Json(shelf_form): Json<UpdateShelfForm>,
) -> Response {
    if let Some(name) = &shelf_form.name {
        return match app_data.shelf_service.update_shelf_name(&shelf_id, &name).await {
            Ok(_) => responses::mutated_shelf(shelf_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    if let Some(layer) = &shelf_form.layer {
        return match app_data.shelf_service.update_shelf_layer(&shelf_id, *layer).await {
            Ok(_) => responses::mutated_shelf(shelf_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    if let Some(room_id) = &shelf_form.room_id {
        return match app_data.shelf_service.update_shelf_room(&shelf_id, *room_id).await {
            Ok(_) => responses::mutated_shelf(shelf_id).into_response(),
            Err(error) => error.into_response(),
        };
    }
    ServiceError::PayloadNotValid.into_response()
}

#[allow(clippy::unused_async)]
pub async fn get_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(shelf_id): Path<ShelfId>,
) -> Response {
    match app_data.shelf_service.get_shelf(&shelf_id).await {
        Ok(shelf) => responses::get_shelf(shelf).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_paged_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Query(criteria): Query<ListingCriteria>,
    Query(extra_room): Query<ExtraRoomId>,
    Query(paged_conf): Query<PagedConf>,
) -> Response {
    if let Some(b) = paged_conf.all {
        if b {
            return match app_data.shelf_service.get_all_shelves(extra_room.room_id).await {
                Ok(shelves) => Json(OkResponseData { data: shelves }).into_response(),
                Err(error) => error.into_response(),
            };
        }
    }
    let spec = app_data.cfg.spec_from_criteria(&criteria).await;
    match app_data.shelf_service.get_shelves(&spec, extra_room.room_id).await {
        Ok(shelves) => Json(OkResponseData { data: shelves }).into_response(),
        Err(error) => error.into_response(),
    }
}
