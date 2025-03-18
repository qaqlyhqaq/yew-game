mod component;

use yew::prelude::*;
use gloo::console;
use crate::component::container::{Container, ContainerLyh};

#[function_component(App)]
fn app() -> Html {
    html! {
        <ContainerLyh/>
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}