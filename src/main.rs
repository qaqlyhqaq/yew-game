mod component;
mod net;

use std::sync::Arc;
use crate::component::container::MainContainer;
use yew::prelude::*;
use crate::net::request::ClientBase;
use crate::net::task_manage::TaskClient;

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
        let client:TaskClient = TaskClient {
            token: Arc::new(Default::default()),
        };
        TaskClient::login().await;
    });
    yew::Renderer::<App>::new().render();
}
