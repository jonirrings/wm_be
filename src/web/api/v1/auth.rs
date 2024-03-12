use std::sync::Arc;

use hyper::http::HeaderValue;

use crate::common::AppData;
use crate::errors::ServiceError;
use crate::models::user::{UserClaims, UserCompact, UserId};
use crate::services::authentication::JsonWebToken;
use crate::web::api::v1::extractors::bearer_token::BearerToken;

pub struct Authentication {
    json_web_token: Arc<JsonWebToken>,
}

impl Authentication {
    #[must_use]
    pub fn new(json_web_token: Arc<JsonWebToken>) -> Self {
        Self { json_web_token }
    }

    /// Create Json Web Token
    pub async fn sign_jwt(&self, user: UserCompact) -> String {
        self.json_web_token.sign(user).await
    }

    /// Verify Json Web Token
    ///
    /// # Errors
    ///
    /// This function will return an error if the JWT is not good or expired.
    pub async fn verify_jwt(&self, token: &str) -> Result<UserClaims, ServiceError> {
        self.json_web_token.verify(token).await
    }

    /// Get logged-in user ID from bearer token
    ///
    /// # Errors
    ///
    /// This function will return an error if it can get claims from the request
    pub async fn get_user_id_from_bearer_token(&self, maybe_token: &Option<BearerToken>) -> Result<UserId, ServiceError> {
        let claims = self.get_claims_from_bearer_token(maybe_token).await?;
        Ok(claims.user.user_id)
    }

    /// Get Claims from bearer token
    ///
    /// # Errors
    ///
    /// This function will:
    ///
    /// - Return an `ServiceError::TokenNotFound` if `HeaderValue` is `None`.
    /// - Pass through the `ServiceError::TokenInvalid` if unable to verify the JWT.
    async fn get_claims_from_bearer_token(&self, maybe_token: &Option<BearerToken>) -> Result<UserClaims, ServiceError> {
        match maybe_token {
            Some(token) => match self.verify_jwt(&token.value()).await {
                Ok(claims) => Ok(claims),
                Err(e) => Err(e),
            },
            None => Err(ServiceError::TokenNotFound),
        }
    }
}

/// Parses the token from the `Authorization` header.
///
/// # Panics
///
/// This function will panic if the `Authorization` header is not a valid `String`.
pub fn parse_token(authorization: &HeaderValue) -> String {
    let split: Vec<&str> = authorization
        .to_str()
        .expect("variable `auth` contains data that is not visible ASCII chars.")
        .split("Bearer")
        .collect();
    let token = split[1].trim();
    token.to_string()
}

/// If the user is logged in, returns the user's ID. Otherwise, returns `None`.
///
/// # Errors
///
/// It returns an error if we cannot get the user from the bearer token.
pub async fn get_optional_logged_in_user(
    maybe_bearer_token: Option<BearerToken>,
    app_data: Arc<AppData>,
) -> Result<Option<UserId>, ServiceError> {
    match maybe_bearer_token {
        Some(bearer_token) => match app_data.auth.get_user_id_from_bearer_token(&Some(bearer_token)).await {
            Ok(user_id) => Ok(Some(user_id)),
            Err(error) => Err(error),
        },
        None => Ok(None),
    }
}
