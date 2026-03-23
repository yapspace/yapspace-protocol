use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub owner: Option<bool>,
    pub owner_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartialGuild {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,
}
