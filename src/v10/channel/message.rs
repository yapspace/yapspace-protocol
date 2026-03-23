use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: i64,
    pub content: String,
    pub author_id: i64,
    pub timestamp: std::time::SystemTime,
    pub edited_timestamp: Option<std::time::SystemTime>,
}
