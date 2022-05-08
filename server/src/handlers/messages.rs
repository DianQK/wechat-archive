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
use crate::database::WaImgInfo;
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
    fn get_message(
        &self,
        display_name_map: &HashMap<String, String>,
        image_map: &HashMap<u64, &WaImgInfo>,
    ) -> Message {
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
                let thumbnail_url = format!(
                    "/assets/image2/{}/{}/th_{}",
                    img_prefix_1, img_prefix_2, img_id
                );
                // TODO: 不存在的图片能否重新下载？
                let url = match image_map.get(&self.msg_svr_id) {
                    Some(image_info) => {
                        let big_img_path = &image_info.big_img_path;
                        let img_prefix_1 = &big_img_path[0..2];
                        let img_prefix_2 = &big_img_path[2..4];
                        format!(
                            "/assets/image2/{img_prefix_1}/{img_prefix_2}/{}",
                            image_info.big_img_path
                        )
                    }
                    None => format!("{thumbnail_url}hd"),
                };
                Content::Image { thumbnail_url, url }
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
        // TODO: 更合适的处理方式？
    }
    let svr_ids: Vec<u64> = messages.iter().map(|m| m.msg_svr_id).collect();
    let images: Vec<WaImgInfo> = RB
        .fetch_list_by_column(WaImgInfo::msg_svr_id(), &svr_ids)
        .await
        .unwrap();
    let mut image_map: HashMap<u64, &WaImgInfo> = HashMap::new();
    for image in images.iter() {
        image_map.insert(image.msg_svr_id, image);
    }
    let messages: Vec<Message> = messages
        .iter()
        .map(|m| m.get_message(&display_name_map, &image_map))
        .collect(); // ? 什么奇怪的写法
    Ok(Json(messages))
}
