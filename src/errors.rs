use std::borrow::Cow;
use std::error;

use derive_more::{Display, Error};
use hyper::StatusCode;

use crate::databases::database;

pub type ServiceResult<V> = Result<V, ServiceError>;

#[derive(Debug, Display, PartialEq, Eq, Error)]
#[allow(dead_code)]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(fmt = "This server is is closed for registration. Contact admin if this is unexpected")]
    ClosedForRegistration,

    #[display(fmt = "Email is required")] //405j
    EmailMissing,
    #[display(fmt = "Please enter a valid email address")] //405j
    EmailInvalid,

    #[display(fmt = "The value you entered for URL is not a URL")] //405j
    NotAUrl,

    #[display(fmt = "Invalid username/email or password")]
    WrongPasswordOrUsername,
    #[display(fmt = "Username not found")]
    UsernameNotFound,
    #[display(fmt = "User not found")]
    UserNotFound,

    #[display(fmt = "Account not found")]
    AccountNotFound,

    /// when the value passed contains profanity
    #[display(fmt = "Can't allow profanity in usernames")]
    ProfanityError,
    /// when the value passed contains blacklisted words
    /// see [blacklist](https://github.com/shuttlecraft/The-Big-Username-Blacklist)
    #[display(fmt = "Username contains blacklisted words")]
    BlacklistError,
    /// when the value passed contains characters not present
    /// in [UsernameCaseMapped](https://tools.ietf.org/html/rfc8265#page-7)
    /// profile
    #[display(fmt = "username_case_mapped violation")]
    UsernameCaseMappedError,

    #[display(fmt = "Password too short")]
    PasswordTooShort,
    #[display(fmt = "Username too long")]
    PasswordTooLong,
    #[display(fmt = "Passwords don't match")]
    PasswordsDontMatch,

    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,

    #[display(fmt = "Username contains illegal characters")]
    UsernameInvalid,

    /// email is already taken
    #[display(fmt = "Email not available")]
    EmailTaken,

    #[display(fmt = "Please verify your email before logging in")]
    EmailNotVerified,

    /// when the a token name is already taken
    /// token not found
    #[display(fmt = "Token not found. Please sign in.")]
    TokenNotFound,

    /// token expired
    #[display(fmt = "Token expired. Please sign in again.")]
    TokenExpired,

    #[display(fmt = "Token invalid.")]
    /// token invalid
    TokenInvalid,

    #[display(fmt = "Some mandatory metadata fields are missing.")]
    MissingMandatoryMetadataFields,

    #[display(fmt = "Unauthorized action.")]
    Unauthorized,

    #[display(fmt = "Could not whitelist torrent.")]
    WhitelistingError,

    #[display(fmt = "Failed to send verification email.")]
    FailedToSendVerificationEmail,

    #[display(fmt = "Database error.")]
    DatabaseError,

    #[display(fmt = "Room not found")]
    RoomNotFound,
    #[display(fmt = "Name not valid, length [4,20)")]
    NameNotValid,
    #[display(fmt = "Description not valid, max length 200")]
    DescNotValid,
    #[display(fmt = "Payload content required")]
    PayloadNotValid,
    #[display(fmt = "Shelf not found")]
    ShelfNotFound,
    #[display(fmt = "Item not found")]
    ItemNotFound,
    #[display(fmt = "Insufficient Item")]
    InsufficientItem,
    #[display(fmt = "Count must be positive")]
    CountMustBePositive,
    #[display(fmt = "Source must be positive")]
    SourceMustBePositive,
    #[display(fmt = "Target must be positive")]
    TargetMustBePositive,
}

impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("{e:?}");
        //fixme: we can tell different error here
        ServiceError::InternalServerError
    }
}

impl From<database::Error> for ServiceError {
    fn from(e: database::Error) -> Self {
        map_database_error_to_service_error(&e)
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(e: argon2::password_hash::Error) -> Self {
        eprintln!("{e}");
        ServiceError::InternalServerError
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(e: std::io::Error) -> Self {
        eprintln!("{e}");
        ServiceError::InternalServerError
    }
}

impl From<Box<dyn error::Error>> for ServiceError {
    fn from(e: Box<dyn error::Error>) -> Self {
        eprintln!("{e}");
        ServiceError::InternalServerError
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(e: serde_json::Error) -> Self {
        eprintln!("{e}");
        ServiceError::InternalServerError
    }
}


#[must_use]
pub fn http_status_code_for_service_error(error: &ServiceError) -> StatusCode {
    #[allow(clippy::match_same_arms)]
    match error {
        ServiceError::ClosedForRegistration => StatusCode::FORBIDDEN,
        ServiceError::EmailInvalid => StatusCode::BAD_REQUEST,
        ServiceError::NotAUrl => StatusCode::BAD_REQUEST,
        ServiceError::WrongPasswordOrUsername => StatusCode::FORBIDDEN,
        ServiceError::UsernameNotFound => StatusCode::NOT_FOUND,
        ServiceError::UserNotFound => StatusCode::NOT_FOUND,
        ServiceError::AccountNotFound => StatusCode::NOT_FOUND,
        ServiceError::ProfanityError => StatusCode::BAD_REQUEST,
        ServiceError::BlacklistError => StatusCode::BAD_REQUEST,
        ServiceError::UsernameCaseMappedError => StatusCode::BAD_REQUEST,
        ServiceError::PasswordTooShort => StatusCode::BAD_REQUEST,
        ServiceError::PasswordTooLong => StatusCode::BAD_REQUEST,
        ServiceError::PasswordsDontMatch => StatusCode::BAD_REQUEST,
        ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,
        ServiceError::UsernameInvalid => StatusCode::BAD_REQUEST,
        ServiceError::EmailTaken => StatusCode::BAD_REQUEST,
        ServiceError::EmailNotVerified => StatusCode::FORBIDDEN,
        ServiceError::TokenNotFound => StatusCode::UNAUTHORIZED,
        ServiceError::TokenExpired => StatusCode::UNAUTHORIZED,
        ServiceError::TokenInvalid => StatusCode::UNAUTHORIZED,
        ServiceError::MissingMandatoryMetadataFields => StatusCode::BAD_REQUEST,
        ServiceError::Unauthorized => StatusCode::FORBIDDEN,
        ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        ServiceError::EmailMissing => StatusCode::NOT_FOUND,
        ServiceError::FailedToSendVerificationEmail => StatusCode::INTERNAL_SERVER_ERROR,
        ServiceError::WhitelistingError => StatusCode::INTERNAL_SERVER_ERROR,
        ServiceError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        ServiceError::RoomNotFound => StatusCode::NOT_FOUND,
        ServiceError::NameNotValid => StatusCode::BAD_REQUEST,
        ServiceError::DescNotValid => StatusCode::BAD_REQUEST,
        ServiceError::PayloadNotValid => StatusCode::BAD_REQUEST,
        ServiceError::ShelfNotFound => StatusCode::NOT_FOUND,
        ServiceError::ItemNotFound => StatusCode::NOT_FOUND,
        ServiceError::InsufficientItem => StatusCode::BAD_REQUEST,
        ServiceError::CountMustBePositive => StatusCode::BAD_REQUEST,
        ServiceError::SourceMustBePositive => StatusCode::BAD_REQUEST,
        ServiceError::TargetMustBePositive => StatusCode::BAD_REQUEST,
    }
}

#[must_use]
pub fn map_database_error_to_service_error(error: &database::Error) -> ServiceError {
    #[allow(clippy::match_same_arms)]
    match error {
        database::Error::Error => ServiceError::InternalServerError,
        database::Error::ErrorWithText(_) => ServiceError::InternalServerError,
        database::Error::UsernameTaken => ServiceError::UsernameTaken,
        database::Error::EmailTaken => ServiceError::EmailTaken,
        database::Error::UserNotFound => ServiceError::UserNotFound,
        database::Error::UnrecognizedDatabaseDriver => ServiceError::InternalServerError,
        database::Error::RoomNotFound => ServiceError::RoomNotFound,
        database::Error::ShelfNotFound => ServiceError::ShelfNotFound,
        database::Error::ItemNotFound => ServiceError::ItemNotFound,
        database::Error::InsufficientItem => ServiceError::InsufficientItem,
        database::Error::CountMustBePositive => ServiceError::CountMustBePositive,
    }
}
