// use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
// use gloo_net::http::Request;
// use waapi::model::{Content, Message};
// use wasm_bindgen_futures::spawn_local;
// use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PreviewImgProps {
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
struct State {
    talker: String,
    page: u64,
}

#[function_component(PreviewImg)]
pub fn previewImg(props: &PreviewImgProps) -> Html {
    html! {
        <img class="preview-img" src={props.thumbnail_url.clone()}/>
    }
}
