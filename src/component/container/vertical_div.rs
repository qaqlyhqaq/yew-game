/*
垂直情况下的 div 容器,内部组件按垂直排布
 */
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}


//display="flex" align-items="center"  height="200px"
#[function_component]
pub fn VerticalDiv(props: &Props) -> Html {

    html! {
        <div style="display: flex; flex-direction: column;"  >
        {props.children.clone()}
        </div>
    }
}