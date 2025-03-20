mod component;

use crate::component::container::{Container, ContainerLyh};
use gloo::console;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ContainerLyh  />
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}
