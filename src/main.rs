mod component;

use yew::prelude::*;
use gloo::console;
use crate::component::container::{ Container};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container/>
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}