use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;

use rbatis::rbatis::Rbatis;
use reqwest::StatusCode;

use crate::{utils, RB};

pub async fn get_conversations(
    Path(username): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conversations: Vec<Conversation> = select_conversations(&RB, &username).await.unwrap(); // TODO: 错误转换
    for conversation in conversations.iter_mut() {
        // TODO: 如何处理 empty null
        if let Some(nickname) = &conversation.nickname {
            if nickname.is_empty() {
                conversation.nickname = None;
            }
        }
        conversation.avatar = Some(utils::get_avatar_path(&conversation.username));
    }
    Ok(Json(conversations))
}

#[crud_table(table_columns:"id,username,alias,con_remark,nickname,msg_count,digest,last_time")]
#[derive(Clone, Debug)]
pub struct Conversation {
    id: u64,
    username: String,
    avatar: Option<String>,
    alias: Option<String>, // TODO: 应该想办法让空字符串为 null？
    con_remark: Option<String>,
    nickname: Option<String>,
    msg_count: u32,
    digest: Option<String>,
    last_time: Option<u64>,
}

#[py_sql("SELECT wa_conversation.id as id, wa_contact.username as username, alias, con_remark, nickname, msg_count , digest, last_time 
FROM wa_conversation 
LEFT JOIN wa_contact 
ON wa_conversation.username = wa_contact.username 
WHERE wa_conversation.owner = #{owner} 
AND wa_contact.wa_owner = #{owner} 
ORDER BY last_time DESC;")]
async fn select_conversations(rb: &Rbatis, owner: &str) -> Vec<Conversation> {
    impled!()
}
