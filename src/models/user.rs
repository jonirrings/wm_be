use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
pub type UserId = i64;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub user_id: UserId,
    pub created_at: Option<String>,
    pub administrator: bool,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserAuthentication {
    pub user_id: UserId,
    pub password_hash: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserProfile {
    pub user_id: UserId,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub avatar: String,
    pub updated_at: Option<String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserCompact {
    pub user_id: UserId,
    pub username: String,
    pub administrator: bool,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserFull {
    pub user_id: UserId,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub administrator: bool,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub avatar: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user: UserCompact,
    pub exp: u64, // epoch in seconds
}