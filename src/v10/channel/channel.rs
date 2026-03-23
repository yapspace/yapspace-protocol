use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(FromRepr, Serialize, Deserialize)]
#[repr(i32)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub r#type: ChannelType,
}
