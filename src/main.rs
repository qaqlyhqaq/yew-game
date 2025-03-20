mod component;

use crate::component::container::MainContainer;
use gloo::console;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <MainContainer  />
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}
