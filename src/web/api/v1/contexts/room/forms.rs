use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddRoomForm {
    pub name: String,
    pub description: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateRoomForm {
    pub name: Option<String>,
    pub description: Option<String>,
}
