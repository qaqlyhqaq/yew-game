/*
垂直情况下的 div 容器,内部组件按垂直排布
 */
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn VerticalDiv(props: &Props) -> Html {

    html! {
        <div >
        <div style="float:left"></div>
        <div style="clear:both"></div>
        {props.children.clone()}
        <div style="clear:left"></div>
        <div style="float:both"></div>
        </div>
    }
}