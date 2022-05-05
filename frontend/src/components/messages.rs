use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessagesProps {
    pub owner: String,
    pub talker: String,
}

// TODO: 复制代码
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Sender {
    pub username: String,
    // pub display_name: String, // Redis?
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Message {
    pub wa_owner: String,
    pub id: u32,
    pub msg_svr_id: u64,
    pub r#type: i32,
    // pub is_send: i32,
    pub create_time: u64,
    pub talker: String,
    pub content: Option<String>,
    pub sender: Sender,
}

#[derive(Properties, PartialEq)]
struct State {
    talker: String,
    page: u64,
}

#[function_component(Messages)]
pub fn messages(props: &MessagesProps) -> Html {
    let node_ref = use_node_ref();
    let state = use_state(|| State {
        talker: props.talker.clone(),
        page: 1,
    });
    let messages = use_state(|| Vec::<Message>::new());
    if state.talker != props.talker {
        state.set(State {
            talker: props.talker.clone(),
            page: 1,
        });
    }
    {
        let node_ref = node_ref.clone();
        let messages = messages.clone();
        let owner = props.owner.clone();
        use_effect_with_deps(
            move |state| {
                let state = state.clone();
                spawn_local(async move {
                    let scroll_el = node_ref.cast::<Element>().unwrap();
                    let scroll_el_scroll_height = scroll_el.scroll_height();
                    let mut result: Vec<Message> = Request::get(&format!(
                        "/api/messages/{}/{}?page={}&size=50",
                        &owner, state.talker, state.page
                    ))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    if state.page != 1 {
                        result.extend_from_slice(&messages);
                    }
                    messages.set(result);
                    if state.page == 1 {
                        scroll_el.set_scroll_top(scroll_el.scroll_height());
                    } else {
                        scroll_el
                            .set_scroll_top(scroll_el.scroll_height() - scroll_el_scroll_height);
                    }
                });
                || {}
            },
            state.clone(),
        );
    }

    // 滚动到顶部自动刷新
    // let node_ref_onscroll = node_ref.clone();
    // let onscroll = move |e: Event| {
    //     if let Some(el) = node_ref_onscroll.cast::<Element>() {
    //         let scroll_top = el.scroll_top();
    //         if scroll_top < 100 {
    //             page.set(*page+1);
    //             log::info!("Update: {:?} {:?}", e, el.scroll_top());
    //         }
    //     }
    // };

    let load_more_onclick = {
        Callback::from(move |_| {
            state.set(State {
                talker: state.talker.clone(),
                page: state.page + 1,
            });
        })
    };

    html! {
        <div class="messages" ref={node_ref}>
            <button onclick={load_more_onclick}>
                { "加载更多" }
            </button>
            {(*messages).iter().map(|message| html!{
                <div class="message" key={message.id.clone()}>
                <figure class="avatar image is-48x48">
                    <img class="is-rounded" src={message.sender.avatar.clone()}/>
                </figure>
                <div class="content">
                    <small>{ &message.sender.username }</small>
                    { message.content.clone().unwrap_or("NULL".to_string()) }
                 </div>
                </div>
            }).collect::<Html>()}
        </div>
    }
}
