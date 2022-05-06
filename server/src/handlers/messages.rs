use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::crud::CRUD;
use rbatis::{rbatis::Rbatis, Page, PageRequest};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use waapi::model::{Content, Message, Sender};

use crate::database::WaMessage;
use crate::database::WaMessageType;
use crate::utils;
use crate::RB;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    page: u64,
    size: u64,
}

impl From<WaMessage> for Message {
    fn from(wa_message: WaMessage) -> Self {
        let sender_username = wa_message.get_sender_username();
        let sender_avatar = utils::get_avatar_path(&sender_username);
        let content: Content = match wa_message.r#type {
            WaMessageType::Text => Content::Text {
                text: wa_message.content.unwrap_or("Oops!!!".to_string()),
            }, // TODO: 日志记录 or Content::Error
            _ => Content::Unknown {
                type_id: wa_message.r#type as i32,
            },
        };
        Self {
            wa_owner: wa_message.wa_owner.clone(),
            id: wa_message.id.unwrap(),
            msg_svr_id: wa_message.msg_svr_id,
            create_time: wa_message.create_time,
            talker: wa_message.talker.clone(),
            content,
            sender: Sender {
                username: sender_username,
                avatar: sender_avatar,
            },
        }
    }
}

pub async fn get_messages(
    Path((owner, talker)): Path<(String, String)>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, StatusCode> {
    let page_request = PageRequest::new(params.page, params.size);
    let sql_wrapper = RB
        .new_wrapper()
        .is_not_null(WaMessage::msg_svr_id())
        .eq(WaMessage::wa_owner(), owner)
        .eq(WaMessage::talker(), talker)
        .order_by(false, &[WaMessage::create_time()]);
    let messages: Page<WaMessage> = RB
        .fetch_page_by_wrapper(sql_wrapper.clone(), &page_request)
        .await
        .unwrap();
    let mut messages = messages.records;
    messages.reverse(); //TODO: 通过 sql offset + asc 直接获取？
    let messages: Vec<Message> = messages.iter().map(|m| Message::from(m.clone())).collect(); // ? 什么奇怪的写法
    Ok(Json(messages))
}
