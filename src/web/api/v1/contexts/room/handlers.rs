use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{self, Multipart, Path, Query, Extension};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use serde::Deserialize;

use super::errors;
use super::responses;
use super::forms::{AddRoomForm, UpdateRoomForm};
use crate::common::AppData;
use crate::errors::ServiceError;
use crate::models::room::{Room, RoomId};
use crate::services::room::ListingRequest;
use crate::web::api::v1::auth::get_optional_logged_in_user;
use crate::web::api::v1::extractors::bearer_token::Extract;
use crate::web::api::v1::responses::OkResponseData;


#[allow(clippy::unused_async)]
pub async fn add_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Json(room_form): Json<AddRoomForm>,
) -> Response {
    /* let opt_user_id = match get_optional_logged_in_user(maybe_bearer_token, app_data.clone()).await {
         Ok(opt_user_id) => opt_user_id,
         Err(error) => return error.into_response(),
     };*/
    match app_data.room_service.add_room(&room_form/*,opt_user_id*/).await {
        Ok(room_id) => responses::mutated_room(room_id).into_response(),
        Err(error) => error.into_response(),
    }
}


#[allow(clippy::unused_async)]
pub async fn delete_handler(Extension(app_data): Extension<Arc<AppData>>,
                            Extract(maybe_bearer_token): Extract,
                            Path(room_id): Path<RoomId>, ) -> Response {
    match app_data.room_service.close_room(&room_id).await {
        Ok(_) => responses::mutated_room(room_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn update_handler(Extension(app_data): Extension<Arc<AppData>>,
                            Extract(maybe_bearer_token): Extract,
                            Path(room_id): Path<RoomId>,
                            Json(room_form): Json<UpdateRoomForm>, ) -> Response {
    if let Some(name) = &room_form.name{
        return match app_data.room_service.update_room_name(&room_id,&name).await {
            Ok(_) => responses::mutated_room(room_id).into_response(),
            Err(error) => error.into_response(),
        }
    }
    if let Some(desc) = &room_form.description{
        return match app_data.room_service.update_room_desc(&room_id,&desc).await {
            Ok(_) => responses::mutated_room(room_id).into_response(),
            Err(error) => error.into_response(),
        }
    }
    ServiceError::PayloadNotValid.into_response()
}

#[allow(clippy::unused_async)]
pub async fn get_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Extract(maybe_bearer_token): Extract,
    Path(room_id): Path<RoomId>,
) -> Response {
    /*let opt_user_id = match get_optional_logged_in_user(maybe_bearer_token, app_data.clone()).await {
        Ok(opt_user_id) => opt_user_id,
        Err(error) => return error.into_response(),
    };*/
    match app_data.room_service.get_room(&room_id/*, opt_user_id*/).await {
        Ok(room) => responses::get_room(room).into_response(),
        Err(error) => error.into_response(),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_all_handler(Extension(app_data): Extension<Arc<AppData>>,
                             Extract(maybe_bearer_token): Extract,
                             Query(criteria): Query<ListingRequest>,) -> Response{
    match app_data.room_service.get_rooms(&criteria).await {
        Ok(torrents_response) => Json(OkResponseData { data: torrents_response }).into_response(),
        Err(error) => error.into_response(),
    }
}