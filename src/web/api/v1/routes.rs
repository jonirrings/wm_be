use std::env;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;

use super::contexts::about::handlers::about_page_handler;
use super::contexts::{about,user,room};
use crate::bootstrap::config::ENV_VAR_CORS_PERMISSIVE;
use crate::common::AppData;

pub const API_VERSION_URL_PREFIX: &str = "v1";

#[allow(clippy::needless_pass_by_value)]
pub fn router(app_data: Arc<AppData>) -> Router {
    let v1_api_routes = Router::new()
        .nest("/user", user::routes::router(app_data.clone()))
        .nest("/rooms", room::routes::router(app_data.clone()))
        .nest("/about", about::routes::router(app_data.clone()));

    let router = Router::new()
        .route("/", get(about_page_handler).with_state(app_data.clone()))
        .route("/health_check", get(health_check_handler).with_state(app_data))
        .nest(&format!("/{API_VERSION_URL_PREFIX}"), v1_api_routes);

    let router = if env::var(ENV_VAR_CORS_PERMISSIVE).is_ok() {
        router.layer(CorsLayer::permissive())
    } else {
        router
    };

    router.layer(DefaultBodyLimit::max(10_485_760)).layer(CompressionLayer::new())
}

/// Endpoint for container health check.
async fn health_check_handler() -> Json<Value> {
    Json(json!({ "status": "Ok" }))
}