use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(clippy::module_name_repetitions)]
pub type CategoryId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Category {
    pub category_id: CategoryId,
    pub name: String,
    pub description: Option<String>,
}