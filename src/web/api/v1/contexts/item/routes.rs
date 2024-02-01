use axum::routing::{delete, get, post};
use axum::Router;

use super::handlers::{
    add_handler, convert_handler, delete_handler, deposit_handler, get_all_handler, get_handler, patch_handler, transfer_handler,
    update_handler, withdraw_handler,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_handler).post(add_handler))
        .route(
            "/:id",
            delete(delete_handler)
                .put(update_handler)
                .patch(patch_handler)
                .get(get_handler),
        )
        .route("/:id/withdraw", post(withdraw_handler))
        .route("/:id/deposit", post(deposit_handler))
        .route("/:id/transfer", post(transfer_handler))
        .route("/convert", post(convert_handler))
}
