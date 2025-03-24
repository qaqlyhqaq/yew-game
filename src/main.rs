mod component;
mod net;

use crate::component::container::MainContainer;
use gloo::console;
use yew::prelude::*;

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
    yew::Renderer::<App>::new().render();
}
