use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use hyper::StatusCode;

use crate::web::api::v1::responses::{json_error_response, ErrorResponseData};

#[derive(Debug, Display, PartialEq, Eq, Error)]
pub enum Request {
    #[display(fmt = "provided ID for Room is not valid.")]
    InvalidRoomId,
    #[display(fmt = "room title bytes are nota valid UTF8 string.")]
    NameIsNotValidUtf8,
    #[display(fmt = "room description bytes are nota valid UTF8 string.")]
    DescriptionIsNotValidUtf8,
}

impl IntoResponse for Request {
    fn into_response(self) -> Response {
        json_error_response(
            http_status_code_for_handler_error(&self),
            &ErrorResponseData { error: self.to_string() },
        )
    }
}

#[must_use]
pub fn http_status_code_for_handler_error(error: &Request) -> StatusCode {
    #[allow(clippy::match_same_arms)]
    match error {
        Request::NameIsNotValidUtf8 => StatusCode::EXPECTATION_FAILED,
        Request::DescriptionIsNotValidUtf8 => StatusCode::EXPECTATION_FAILED,
        Request::InvalidRoomId => StatusCode::BAD_REQUEST,
        // Internal errors processing the request
    }
}