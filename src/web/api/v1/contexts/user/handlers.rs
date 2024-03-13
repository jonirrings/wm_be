use std::sync::Arc;

use axum::extract::{Extension, Host, Path};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;

use crate::common::AppData;
use crate::web::api::v1::extractors::bearer_token::Extract;
use crate::web::api::v1::responses::OkResponseData;

use super::forms::{JsonWebToken, LoginForm, RegistrationForm};
use super::responses;

// Registration

/// It handles the registration of a new user.
///
/// # Errors
///
/// It returns an error if the user could not be registered.
#[allow(clippy::unused_async)]
pub async fn registration_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Host(host_from_header): Host,
    Json(registration_form): Json<RegistrationForm>,
) -> Response {
    let api_base_url = app_data
        .cfg
        .get_api_base_url()
        .await
        .unwrap_or(api_base_url(&host_from_header));

    match app_data
        .registration_service
        .register_user(&registration_form, &api_base_url)
        .await
    {
        Ok(user_id) => responses::added_user(user_id).into_response(),
        Err(error) => error.into_response(),
    }
}

#[derive(Deserialize)]
pub struct TokenParam(String);

/// It handles the verification of the email verification token.
#[allow(clippy::unused_async)]
pub async fn email_verification_handler(Extension(app_data): Extension<Arc<AppData>>, Path(token): Path<TokenParam>) -> String {
    match app_data.registration_service.verify_email(&token.0).await {
        Ok(_) => String::from("Email verified, you can close this page."),
        Err(error) => error.to_string(),
    }
}

// Authentication

/// It handles the user login.
///
/// # Errors
///
/// It returns an error if:
///
/// - Unable to verify the supplied payload as a valid JWT.
/// - The JWT is not invalid or expired.
#[allow(clippy::unused_async)]
pub async fn login_handler(Extension(app_data): Extension<Arc<AppData>>, Json(login_form): Json<LoginForm>) -> Response {
    match app_data
        .authentication_service
        .login(&login_form.username, &login_form.password)
        .await
    {
        Ok((token, user_compact)) => responses::logged_in_user(token, user_compact).into_response(),
        Err(error) => error.into_response(),
    }
}

/// Who am I
///
/// # Errors
///
/// It returns an error if:
///
/// - Unable to verify the JWT
#[allow(clippy::unused_async)]
pub async fn who_am_i_handler(Extension(app_data): Extension<Arc<AppData>>, Extract(maybe_bearer_token): Extract) -> Response {
    let user_id = match app_data.auth.get_user_id_from_bearer_token(&maybe_bearer_token).await {
        Ok(user_id) => user_id,
        Err(error) => return error.into_response(),
    };
    match app_data.registration_service.get_user_by_id(user_id).await {
        Ok(u) => Json(OkResponseData { data: u }).into_response(),
        Err(error) => error.into_response(),
    }
}

/// It verifies a supplied JWT.
///
/// # Errors
///
/// It returns an error if:
///
/// - Unable to verify the supplied payload as a valid JWT.
/// - The JWT is not invalid or expired.
#[allow(clippy::unused_async)]
pub async fn verify_token_handler(Extension(app_data): Extension<Arc<AppData>>, Json(token): Json<JsonWebToken>) -> Response {
    match app_data.json_web_token.verify(&token.token).await {
        Ok(_) => Json(OkResponseData {
            data: "Token is valid.".to_string(),
        })
        .into_response(),
        Err(error) => error.into_response(),
    }
}

#[derive(Deserialize)]
pub struct UsernameParam(pub String);

/// It renews the JWT.
///
/// # Errors
///
/// It returns an error if:
///
/// - Unable to parse the supplied payload as a valid JWT.
/// - The JWT is not invalid or expired.
#[allow(clippy::unused_async)]
pub async fn renew_token_handler(Extension(app_data): Extension<Arc<AppData>>, Json(token): Json<JsonWebToken>) -> Response {
    match app_data.authentication_service.renew_token(&token.token).await {
        Ok((token, user_compact)) => responses::renewed_token(token, user_compact).into_response(),
        Err(error) => error.into_response(),
    }
}

/// It bans a user from the index.
///
/// # Errors
///
/// This function will return if:
///
/// - The JWT provided by the banning authority was not valid.
/// - The user could not be banned: it does not exist, etcetera.
#[allow(clippy::unused_async)]
pub async fn ban_handler(
    Extension(app_data): Extension<Arc<AppData>>,
    Path(to_be_banned_username): Path<UsernameParam>,
    Extract(maybe_bearer_token): Extract,
) -> Response {
    // todo: add reason and `date_expiry` parameters to request

    let user_id = match app_data.auth.get_user_id_from_bearer_token(&maybe_bearer_token).await {
        Ok(user_id) => user_id,
        Err(error) => return error.into_response(),
    };

    match app_data.ban_service.ban_user(&to_be_banned_username.0, &user_id).await {
        Ok(()) => Json(OkResponseData {
            data: format!("Banned user: {}", to_be_banned_username.0),
        })
        .into_response(),
        Err(error) => error.into_response(),
    }
}

/// It returns the base API URL without the port. For example: `http://localhost`.
fn api_base_url(host: &str) -> String {
    // HTTPS is not supported yet.
    format!("http://{host}")
}
