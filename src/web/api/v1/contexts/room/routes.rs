use std::sync::Arc;

use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};

use super::handlers::{add_handler, delete_handler, update_handler, get_handler, get_all_handler};
use crate::common::AppData;

pub fn router() -> Router{
    Router::new()
        .route("/", get(get_all_handler))
        .route("/", post(add_handler))
        .route("/:id", delete(delete_handler))
        .route("/:id", put(update_handler))
        .route("/:id", get(get_handler))
}