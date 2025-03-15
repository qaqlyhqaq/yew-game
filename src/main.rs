use yew::prelude::*;

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
    yew::Renderer::<App>::new().render();
}