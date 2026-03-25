use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::v10::user::PartialUser;

#[derive(FromRepr, Serialize, Deserialize)]
#[repr(i32)]
pub enum RelationshipType {
    None = 0,
    Friend = 1,
    Blocked = 2,
    IncomingRequest = 3,
    OutgoingRequest = 4,
    Implicit = 5,
}

#[derive(Serialize, Deserialize)]
pub struct Relationship {
    /// The ID of the target user
    pub id: String,
    /// The type of relationship
    pub r#type: RelationshipType,
    /// The target user
    pub user: PartialUser,
    /// The nickname of the user in this relationship (1-32 characters)
    pub nickname: Option<String>,
    /// Whether the friend request was flagged as spam (default false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spam_request: Option<bool>,
    /// Whether the friend request was sent by a user without a mutual friend or small mutual guild (default false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stranger_request: Option<bool>,
    /// Whether the target user has been ignored by the current user
    pub user_ignored: bool,
    /// When the user requested a relationship (ISO8601 timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SendFriendRequest {
    pub username: String,
    pub discriminator: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRelationship {
    /// If left out, will default to -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RelationshipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_friend_suggestion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm_stranger_request: Option<bool>,
}
