use axum::response::IntoResponse;
use axum::Json;
use rbatis::crud::CRUD;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::WaUserInfo;
use crate::utils;
use crate::RB;

#[derive(Debug, Deserialize, Serialize, Default)]
struct User {
    username: String,
    alias: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
}

impl User {
    fn new(user_info: &WaUserInfo) -> Self {
        let avatar = utils::get_avatar_path(&user_info.username);
        User {
            username: user_info.username.clone(),
            alias: user_info.alias.clone(),
            nickname: user_info.nickname.clone(),
            avatar: Some(avatar),
        }
    }
}

pub async fn get_users() -> Result<impl IntoResponse, StatusCode> {
    let users: Vec<WaUserInfo> = RB.fetch_list().await.unwrap_or_default();
    let users: Vec<User> = users.iter().map(|user| User::new(user)).collect();
    Ok(Json(users))
}
