use axum::routing::{delete, get};
use axum::{Router};

use super::handlers::{add_handler, delete_handler, update_handler, get_handler, get_all_handler};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_handler).post(add_handler))
        .route("/:id", delete(delete_handler).patch(update_handler).get(get_handler))
}