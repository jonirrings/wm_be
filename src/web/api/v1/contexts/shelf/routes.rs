use axum::routing::{delete, get};
use axum::Router;

use super::handlers::{
    add_handler, batch_delete_handler, delete_handler, get_handler, get_paged_handler, patch_handler, update_handler,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_paged_handler).post(add_handler).delete(batch_delete_handler))
        .route(
            "/:id",
            delete(delete_handler)
                .put(update_handler)
                .patch(patch_handler)
                .get(get_handler),
        )
}
