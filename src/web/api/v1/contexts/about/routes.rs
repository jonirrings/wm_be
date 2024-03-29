//! API routes for the [`about`](crate::web::api::v1::contexts::about) API context.
//!
//! Refer to the [API endpoint documentation](crate::web::api::v1::contexts::about).
use axum::routing::get;
use axum::Router;

use super::handlers::{about_page_handler, license_page_handler};

/// Routes for the [`about`](crate::web::api::v1::contexts::about) API context.
pub fn router() -> Router {
    Router::new()
        .route("/about", get(about_page_handler))
        .route("/license", get(license_page_handler))
}
