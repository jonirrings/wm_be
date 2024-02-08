use super::forms::{AddItemForm, UpdateItemForm};
use super::responses;
use crate::common::{AppData, ListingCriteria, PagedConf};
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
pub async fn batch_delete_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(ids): Json<Vec<i64>>,
) -> Response {
    match app_data.item_service.remove_items(&ids).await {
        Ok(res) => Json(res).into_response(),
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
pub async fn get_paged_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Query(criteria): Query<ListingCriteria>,
    Query(paged_conf): Query<PagedConf>,
) -> Response {
    if let Some(b) = paged_conf.all {
        if b {
            return match app_data.item_service.get_all_items().await {
                Ok(items) => Json(OkResponseData { data: items }).into_response(),
                Err(error) => error.into_response(),
            };
        }
    }
    let spec = app_data.cfg.spec_from_criteria(&criteria).await;
    match app_data.item_service.get_items(&spec).await {
        Ok(items) => Json(OkResponseData { data: items }).into_response(),
        Err(error) => error.into_response(),
    }
}
