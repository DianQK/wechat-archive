use std::collections::HashMap;

use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::crud::CRUD;
use rbatis::{Page, PageRequest};
use reqwest::StatusCode;
use serde::Deserialize;

use waapi::model::{Content, Message, Sender};

use crate::database::WaContact;
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

impl WaMessage {
    fn get_message(&self, display_name_map: &HashMap<String, String>) -> Message {
        let sender_username = self.get_sender_username();
        let sender_avatar = utils::get_avatar_path(&sender_username);
        let content: Content = match self.r#type {
            WaMessageType::Text => Content::Text {
                text: self.content.clone().unwrap_or("Oops!!!".to_string()),
            }, // TODO: 日志记录 or Content::Error
            WaMessageType::Image => {
                let img_path = &self.img_path.as_ref().unwrap();
                let img_id = &img_path[23..img_path.len()];
                let img_prefix_1 = &img_id[0..2];
                let img_prefix_2 = &img_id[2..4];
                // TODO: 不存在的图片能否重新下载？
                Content::Image {
                    thumbnail_url: format!(
                        "/assets/image2/{}/{}/th_{}",
                        img_prefix_1, img_prefix_2, img_id
                    ),
                    url: format!(
                        "/assets/image2/{}/{}/{}.jpg",
                        img_prefix_1, img_prefix_2, img_id
                    ), // TODO: 从 ImgeInfo2 中查询
                }
            }
            WaMessageType::Emoji => Content::Emoji,
            _ => Content::Unknown {
                type_id: self.r#type as i32,
            },
        };
        Message {
            wa_owner: self.wa_owner.clone(),
            id: self.id.unwrap(),
            msg_svr_id: self.msg_svr_id,
            create_time: self.create_time,
            talker: self.talker.clone(),
            content,
            sender: Sender {
                username: sender_username.clone(),
                display_name: display_name_map[&sender_username].clone(),
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
    let usernames: Vec<String> = messages.iter().map(|m| m.get_sender_username()).collect();
    let mut display_name_map: HashMap<String, String> = HashMap::new();
    let contacts: Vec<WaContact> = RB
        .fetch_list_by_column(WaContact::username(), &usernames)
        .await
        .unwrap();
    for contact in contacts {
        display_name_map.insert(contact.username.clone(), contact.wa_display_name.clone());
    }
    let messages: Vec<Message> = messages
        .iter()
        .map(|m| m.get_message(&display_name_map))
        .collect(); // ? 什么奇怪的写法
    Ok(Json(messages))
}
