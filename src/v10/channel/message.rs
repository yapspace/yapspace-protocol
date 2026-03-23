use serde::{Deserialize, Serialize};

pub struct Message {
    pub id: i64,
    pub channel_id: i64,
    pub content: String,
    pub author_id: i64,
    pub timestamp: std::time::SystemTime,
    pub edited_timestamp: Option<std::time::SystemTime>,
}
