use yew::prelude::*;
use gloo::console;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "This is apply !" }</h1>
            <img src="static/apple.jpg" />
        </div>
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}