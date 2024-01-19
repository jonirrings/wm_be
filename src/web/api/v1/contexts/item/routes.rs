use axum::routing::{delete, get, post};
use axum::{Router};

use super::handlers::{add_handler, delete_handler, update_handler, get_handler, get_all_handler,
                      transfer_handler, withdraw_handler, deposit_handler, convert_handler};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_handler).post(add_handler))
        .route("/:id", delete(delete_handler).patch(update_handler).get(get_handler))
        .route("/:id/withdraw", post(withdraw_handler))
        .route("/:id/deposit", post(deposit_handler))
        .route("/:id/transfer", post(transfer_handler))
        .route("/convert", post(convert_handler))
}