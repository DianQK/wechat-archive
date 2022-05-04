use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use rbatis::crud::CRUD;
use rbatis::{rbatis::Rbatis, Page, PageRequest};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::WaMessage;
use crate::utils;
use crate::RB;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    page: u64,
    size: u64,
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
    Ok(Json(messages))
}
