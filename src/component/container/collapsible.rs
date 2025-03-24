/*
可折叠容器组件
 */
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};


#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub title:String,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub button_back_call: Callback<()>
}


#[function_component]
pub fn Collapsible(props: &Props) -> Html {
    // 状态管理：是否展开
    let is_expanded = use_state(|| false);

    // 点击事件处理
    let ontoggle = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    // 动态类名
    let content_class = if *is_expanded {
        classes!("collapse-content", "expanded")
    } else {
        classes!("collapse-content")
    };

    // 箭头图标
    let arrow = if *is_expanded { "▼" } else { "▶" };

    let header_text = vec!["任务编号","任务名称","优先级","所属项目","项目阶段","负责人","起止时间","任务进度","状态","操作"];

    let mut body_vec = vec![];
    for i in 0..10{
        body_vec.push(header_text.clone())
    }
    let button_call_back = props.clone().button_back_call.clone();

    html! {
        <div >
            <div  style="text-align:right;min-width:600px;" >
                <span onclick={ontoggle}  style="float:left;"  class="arrow">{ arrow }</span>
                <span style="float:left;" class="title">{ &props.title }</span>
                <button  >{"查询结果"}</button>
            </div>
            <div style="min-width:600px;" class={content_class}>
                {
                    if *is_expanded {
                        //展开
                        props.children.clone()
                    }else{
                        //关闭
                        html!{}
                    }
                }
            </div>
            <div style="text-align:right;min-width:600px;">
                <span  style="float:left;" >{"数据列表"}</span>
                <button onclick={Callback::from(move |x| {
                        button_call_back.emit(());
                })} >{"添加任务"}</button>
            </div>
        </div>
    }
}


