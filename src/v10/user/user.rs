use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PartialUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}
