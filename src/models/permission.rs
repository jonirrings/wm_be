use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use super::role::RoleId;

#[allow(clippy::module_name_repetitions)]
pub type PermId = i64;
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Permission {
    pub perm_id: PermId,
    pub name: String,
    pub description: Option<String>,
    pub role_id:RoleId
}