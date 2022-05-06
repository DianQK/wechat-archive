// use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
// use gloo_net::http::Request;
// use waapi::model::{Content, Message};
// use wasm_bindgen_futures::spawn_local;
// use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PreviewImgProps {
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
struct State {
    showing_big_img: bool,
    class: String,
    url: String,
}

#[function_component(PreviewImg)]
pub fn previewImg(props: &PreviewImgProps) -> Html {
    let state = use_state(|| State {
        showing_big_img: false,
        class: "preview-img".to_string(),
        url: props.thumbnail_url.clone(),
    });
    let onclick = {
        let state = state.clone();
        let props = props.clone();
        Callback::from(move |_| {
            if state.showing_big_img {
                state.set(State {
                    showing_big_img: false,
                    class: "preview-img".to_string(),
                    url: props.thumbnail_url.clone(),
                });
            } else {
                state.set(State {
                    showing_big_img: true,
                    class: "big-img".to_string(),
                    url: props.url.clone(),
                });
            }
        })
    };
    html! {
        <img  onclick={onclick} class={classes!(state.class.clone())} src={state.url.clone()}/>
    }
}
