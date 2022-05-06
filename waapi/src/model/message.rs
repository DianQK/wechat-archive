use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Sender {
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Message {
    pub wa_owner: String,
    pub id: u32,
    pub msg_svr_id: u64,
    pub r#type: i32,
    pub create_time: u64,
    pub talker: String,
    pub content: Option<String>,
    pub sender: Sender,
}
