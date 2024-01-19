use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(clippy::module_name_repetitions)]
pub type RoleId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Role {
    pub role_id: RoleId,
    pub name: String,
    pub description: Option<String>,
}