use yew::prelude::*;
use yew_router::{history::History, hooks::use_history, prelude::RouterScopeExt};

use crate::router::Route;

pub struct HomeComponent;

impl Component for HomeComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        // let history = use_history().unwrap();
        let onclick: Callback<MouseEvent> = Callback::once(move |_| history.push(Route::OtherComponent));
        html! {
            <div class="container">
                <p>{ "Home" }</p>
                <button {onclick}>{ "Go to other" }</button>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}
