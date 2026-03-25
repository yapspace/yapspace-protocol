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

#[derive(Serialize, Deserialize)]
pub struct AllowedMentions {
    /// Each option can either be "users", "roles", or "everyone"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<String>>,
    /// A list of IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// A list of IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
}

#[derive(Serialize, Deserialize)]
pub struct GetChannelMessages {
    pub limit: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl Default for GetChannelMessages {
    fn default() -> Self {
        Self {
            limit: 50,

            around: None,
            before: None,
            after: None,
        }
    }
}
