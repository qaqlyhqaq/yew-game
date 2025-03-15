use yew::prelude::*;
use gloo::console;

#[function_component(App)]
fn app() -> Html {

    let items = vec!["Hello", "World"];
    html! {
        <>
        <ul>
            { for items.iter().map(|item| html! {
                <li><img src="static/apple.jpg" />{ item }</li>
            })
            }
        </ul>
        </>
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}