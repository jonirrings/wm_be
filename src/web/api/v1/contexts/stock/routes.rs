use super::handlers::{
    convert_handler, deposit_handler, get_items_in_room_handler, get_items_in_rooms_handler, get_items_on_shelf_handler,
    get_items_on_shelves_handler, transfer_handler, withdraw_handler,
};
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/shelf", get(get_items_on_shelves_handler))
        .route("/shelf/:id", get(get_items_on_shelf_handler))
        .route("/room", get(get_items_in_rooms_handler))
        .route("/room/:id", get(get_items_in_room_handler))
        .route("/withdraw", delete(withdraw_handler))
        .route("/deposit", post(deposit_handler))
        .route("/transfer", patch(transfer_handler))
        .route("/convert", patch(convert_handler))
}
