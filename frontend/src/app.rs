use crate::components::{Conversations, Messages, Owners};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/owner/:owner/*")]
    Owner { owner: String },
}

#[derive(Clone, Routable, PartialEq)]
pub enum ConversationRoute {
    #[at("/owner/:owner/conversations")]
    Conversations { owner: String },
    #[at("/owner/:owner/conversation/:talker")]
    Conversation { owner: String, talker: String },
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {
           <div>
               { "未选择用户" }
           </div>
        },
        AppRoute::Owner { owner } => html! {
            <>
            <Conversations username={owner.clone()}></Conversations>
            <Switch<ConversationRoute> render={Switch::render(switch_conversation)} />
            </>
        },
    }
}

fn switch_conversation(route: &ConversationRoute) -> Html {
    match route {
        ConversationRoute::Conversations { .. } => html! {<h1></h1>},
        ConversationRoute::Conversation { owner, talker } => html! {
            <Messages owner={owner.clone()} talker={talker.clone()}/>
        },
    }
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <BrowserRouter>
                    <Owners />
                  <Switch<AppRoute> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }
}
