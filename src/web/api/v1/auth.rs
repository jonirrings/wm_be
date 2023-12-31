use std::sync::Arc;

use hyper::http::HeaderValue;

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