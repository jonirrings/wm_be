use std::env;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde_json::{json, Value};
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;

use crate::bootstrap::config::ENV_VAR_CORS_PERMISSIVE;
use crate::common::AppData;
use crate::web::api::v1::contexts::evt::sse::sse_handler;
use crate::web::api::v1::contexts::evt::wss::ws_handler;

//fixme we may use tower_http::auth layer
use super::contexts::{about, item, room, shelf, user};

pub const API_VERSION_URL_PREFIX: &str = "api/v1";

async fn fallback() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({ "code": 404,"message":"Not Found" })))
}

#[allow(clippy::needless_pass_by_value)]
pub fn router(app_data: Arc<AppData>) -> Router {
    let v1_api_routes = Router::new()
        .nest("/user", user::routes::router())
        .nest("/rooms", room::routes::router())
        .nest("/shelf", shelf::routes::router())
        .nest("/items", item::routes::router());

    let router = Router::new()
        .route("/health_check", get(health_check_handler))
        .route("/sse", get(sse_handler))
        .route("/wss", get(ws_handler))
        .nest("/info", about::routes::router())
        .nest(&format!("/{API_VERSION_URL_PREFIX}"), v1_api_routes)
        .fallback(fallback);

    let router = if env::var(ENV_VAR_CORS_PERMISSIVE).is_ok() {
        router.layer(CorsLayer::permissive())
    } else {
        router
    };

    router
        .layer(DefaultBodyLimit::max(10_485_760))
        .layer(CompressionLayer::new())
        .layer(Extension(app_data))
}

/// Endpoint for container health check.
async fn health_check_handler() -> Json<Value> {
    Json(json!({ "status": "Ok" }))
}
