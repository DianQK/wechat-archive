use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Content {
    Unknown { type_id: i32 },
    Text { text: String },
    Image { thumbnail_url: String , url: String },
    Emoji,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sender {
    pub username: String,
    pub display_name: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub wa_owner: String,
    pub id: u32,
    pub msg_svr_id: u64,
    // pub r#type: i32,
    pub create_time: u64,
    pub talker: String,
    pub content: Content,
    pub sender: Sender,
}

impl Message {

}