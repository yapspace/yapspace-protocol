use serde::{Deserialize, Serialize};

use crate::v10::user::User;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageAuthor {
    Author(User),
    Webhook(WebhookAuthor),
}

#[derive(Serialize, Deserialize)]
pub struct WebhookAuthor {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: i64,
    pub author: MessageAuthor,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
}
