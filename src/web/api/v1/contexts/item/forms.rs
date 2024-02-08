use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddItemForm {
    pub name: String,
    pub description: Option<String>,
    pub sn: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateItemForm {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sn: Option<String>,
}

