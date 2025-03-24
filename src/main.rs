mod component;
mod net;

use crate::component::container::MainContainer;
use gloo::console;
use yew::prelude::*;
use crate::net::request;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <span>{"任务管理"}</span>
            <MainContainer  />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    wasm_bindgen_futures::spawn_local(async move {
        request::ClientBase::login().await.to_owned();
    });
    yew::Renderer::<App>::new().render();
}
