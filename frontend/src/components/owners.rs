use crate::app::ConversationRoute;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*; // 统一路由的代码

// 和 server 重复的代码
#[derive(Debug, Deserialize, Serialize, Default)]
struct User {
    username: String,
    alias: Option<String>,
    nickname: Option<String>,
    avatar: Option<String>,
}

#[function_component(Owners)]
pub fn owners() -> Html {
    let users = use_state(Vec::<User>::new);
    {
        let users = users.clone();
        use_effect(move || {
            if users.is_empty() {
                spawn_local(async move {
                    let resp: Vec<User> = Request::get("/api/users")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    if !resp.is_empty() {
                        users.set(resp);
                    }
                });
            }
            || {}
        });
    }
    html! {
        <div class="owners">
            { (*users).iter().map(|user| html!{
                <Link<ConversationRoute> to={ConversationRoute::Conversations { owner: user.username.clone() }}>
                    <figure class="image is-48x48">
                        <img class="is-rounded" src={user.avatar.clone()}/>
                    </figure>
                </Link<ConversationRoute>>
            }).collect::<Html>()
            }
        </div>
    }
}
