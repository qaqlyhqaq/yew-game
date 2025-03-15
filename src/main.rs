use yew::prelude::*;
use gloo::console;

use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    
    let items = vec!["Hello", "World"];
    html! {
        // <div>
        //     <h1>{ "This is apply !" }</h1>
        //     <img src="static/apple.jpg" />
        // </div>
        <ul>
            { for items.iter().map(|item| html! { 
                <li><img src="static/apple.jpg" />{ item }</li> 
            }) 
            }
        </ul>
    }
}

fn main() {
    console::log!("main start !");
    yew::Renderer::<App>::new().render();
}