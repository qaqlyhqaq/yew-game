/*
自实现tr 组件
 */
use yew::{function_component, html, Html, NodeRef, Properties};


#[derive(Properties, Debug, PartialEq)]
pub struct TrProperties {
}

#[function_component(TrConponent)]
pub fn tr_children_component() -> Html {


    let header_text = vec!["任务编号","任务名称","优先级","所属项目","项目阶段","负责人","起止时间","任务进度","状态","操作"];

    let mut body_vec = vec![];
    for i in 0..10{
        body_vec.push(header_text.clone())
    }
    html! {
             header_text.into_iter()
                  .map(|x|{html!{<td style="border: 1px solid #999; padding: 8px;">{x}</td>}})
                  .collect::<Html>()
    }
}