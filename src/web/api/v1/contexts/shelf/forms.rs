use serde_derive::{Deserialize, Serialize};
use crate::models::room::RoomId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddShelfForm {
    pub name: String,
    pub layer: i64,
    pub room_id: RoomId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateShelfForm {
    pub name: Option<String>,
    pub layer: Option<i64>,
    pub room_id: Option<RoomId>,
}