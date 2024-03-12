use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(clippy::module_name_repetitions)]
pub type FileId = i64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct File {
    pub file_id: FileId,
    pub name: String,
    pub description: Option<String>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
}
