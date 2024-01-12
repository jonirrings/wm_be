use std::sync::Arc;

use axum::routing::{delete, get, post, put};
use axum::Router;

use super::handlers::{add_handler, delete_handler, update_handler, get_handler, get_all_handler};
use crate::common::AppData;

pub fn router(app_data: Arc<AppData>) -> Router{
    Router::new()
        .route("/", get(get_all_handler).with_state(app_data.clone()))
        .route("/", post(add_handler).with_state(app_data.clone()))
        .route("/:id", delete(delete_handler).with_state(app_data.clone()))
        .route("/:id", put(update_handler).with_state(app_data.clone()))
        .route("/:id", get(get_handler).with_state(app_data))
}