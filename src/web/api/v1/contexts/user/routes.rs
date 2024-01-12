use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;

use super::handlers::{
    ban_handler, email_verification_handler, login_handler, registration_handler, renew_token_handler, verify_token_handler,
};
use crate::common::AppData;

pub fn router(app_data: Arc<AppData>) -> Router {
    Router::new()
        .route("/register", post(registration_handler).with_state(app_data.clone()))
        .route("/email/verify/:token", get(email_verification_handler).with_state(app_data.clone()))
        .route("/login", post(login_handler).with_state(app_data.clone()))
        .route("/token/verify", post(verify_token_handler).with_state(app_data.clone()))
        .route("/token/renew", post(renew_token_handler).with_state(app_data.clone()))
        .route("/ban/:user", delete(ban_handler).with_state(app_data))
}