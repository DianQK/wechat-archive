use crate::app::ConversationRoute;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*; // 统一路由的代码

// 和 server 重复的代码
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Conversation {
    id: u64,
    username: String,
    avatar: String,
    alias: Option<String>,
    con_remark: Option<String>,
    nickname: Option<String>,
    msg_count: u32,
    digest: String,
    last_time: Option<u64>,
}

impl Conversation {
    fn display_name(&self) -> String {
        match &self.nickname {
            Some(nickname) => nickname.clone(),
            None => self.username.clone(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ConversationsProps {
    pub username: String,
}

#[function_component(Conversations)]
pub fn conversations(props: &ConversationsProps) -> Html {
    let conversations = use_state(|| Vec::<Conversation>::new());
    {
        let conversations = conversations.clone();
        let username = props.username.clone();
        use_effect(move || {
            if conversations.is_empty() {
                spawn_local(async move {
                    let resp: Vec<Conversation> =
                        Request::get(&format!("/api/conversations/{}", username))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .expect("Conversation 解析失败");
                    conversations.set(resp);
                });
            }
            || {}
        });
    }
    html! {
        <div class="conversations">
            {(*conversations).iter().map(|conversation| html!{
                <Link<ConversationRoute> to={ConversationRoute::Conversation { owner: props.username.to_string(), talker: conversation.username.clone() }}>
                <div class="conversation">
                    <figure class="image is-48x48">
                        <img class="is-rounded" src={conversation.avatar.clone()}/>
                    </figure>
                    <div class="content">
                        <strong> { &conversation.display_name() } </strong>
                        <div> { &conversation.digest } </div>
                    </div>
                </div>
                </Link<ConversationRoute>>
                }).collect::<Html>()
            }
        </div>
    }
}
