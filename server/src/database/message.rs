use log::error;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(i32)]
pub enum WaMessageType {
    UN1879048186 = -1879048186,
    Text = 1,
    Image = 3,
    Voice = 34,
    NameCard = 42,
    Video = 43,
    Emoji = 47,
    Location = 48,
    Link = 49, // https://github.com/ppwwyyxx/wechat-dump/issues/52
    Voip = 50,
    // WxVideo = 62,
    U85 = 85,
    System = 10000,
    U1048625 = 1048625,
    U16777265 = 16777265,
    U268445456 = 268445456,
    U285212721 = 285212721,
    U318767153 = 318767153,
    U419430449 = 419430449,
    U436207665 = 436207665,
    U469762097 = 469762097,
    U570425393 = 570425393,
    U754974769 = 754974769,
    U822083633 = 822083633,
    U922746929 = 922746929,
    U973078577 = 973078577,
    U1090519089 = 1090519089,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct WaMessage {
    pub wa_owner: String,
    // pub msg_id: i32,
    pub id: Option<u32>,
    pub msg_svr_id: u64,
    pub r#type: WaMessageType,
    pub status: Option<i32>,
    pub is_send: i32,
    pub is_show_timer: Option<i32>,
    pub create_time: u64,
    pub talker: String,
    pub content: Option<String>,
    pub img_path: Option<String>,
    pub reserved: Option<String>,
    pub lvbuffer: Option<rbatis::Bytes>,
    pub trans_content: Option<String>,
    pub trans_brand_wording: Option<String>,
    pub talker_id: Option<i32>,
    pub biz_client_msg_id: Option<String>,
    pub biz_chat_id: Option<i32>,
    pub biz_chat_user_id: Option<String>,
    pub msg_seq: Option<i32>,
    pub flag: Option<i32>,
    pub solitaire_fold_info: Option<rbatis::Bytes>,
    pub history_id: Option<String>,
}

impl_field_name_method!(WaMessage {
    wa_owner,
    msg_svr_id,
    talker,
    create_time
});

impl WaMessage {
    pub fn is_owner_send(&self) -> bool {
        match self.is_send {
            1 => true,
            0 => false,
            _ => {
                error!("is_send 非法数值");
                false
            }
        }
    }

    pub fn get_digest(&self) -> String {
        // 添加 type enum
        match self.r#type {
            WaMessageType::Text => self._get_text_digest(),
            WaMessageType::Image => "[图片]".to_string(),
            _ => "[TODO]".to_string(),
        }
    }

    // TODO: 如何支持各种类型？
    pub fn get_text_content(&self) -> Option<String> {
        // 添加 type enum
        match self.r#type {
            WaMessageType::Text => self.content.clone(), // TODO: 移除 username
            WaMessageType::Image => Some("[图片]".to_string()),
            _ => Some("[TODO]".to_string()),
        }
    }

    pub fn is_chatroom(&self) -> bool {
        self.talker.ends_with("@chatroom")
    }

    pub fn get_sender_username(&self) -> String {
        if self.is_send == 1 {
            return self.wa_owner.clone();
        }
        if self.is_chatroom() {
            let content = &self.content.as_ref().unwrap();
            if let Some(index) = content.find(':') {
                return content[0..index].to_string();
            } else {
                // error
                return "".to_string();
            }
        } else {
            return self.talker.clone();
        }
    }

    fn _get_text_digest(&self) -> String {
        match &self.content {
            Some(content) => {
                let chars: Vec<char> = content.chars().collect();
                if chars.len() > 50 {
                    chars[0..50].iter().collect::<String>()
                } else {
                    content.to_string()
                }
            }
            None => "".to_string(),
        }
    }
}

impl WaMessage {
    // TODO: 正确的操作是什么？
    #[py_sql(
        "SELECT *
    FROM wa_message as a, (SELECT MAX(create_time) as create_time, talker
                        FROM wa_message GROUP BY talker) as b
    WHERE a.create_time = b.create_time AND
    a.talker = b.talker AND a.wa_owner = #{owner}"
    )]
    async fn select_latest_messages(rb: &Rbatis, owner: &str) -> Vec<WaMessage> {
        impled!()
    }
}
