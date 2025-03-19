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

    html! {
        <div >
            <div  onclick={ontoggle}>
                <span class="arrow">{ arrow }</span>
                <span class="title">{ &props.title }</span>
            </div>
            <div class={content_class}>
                {
                    if *is_expanded {
                    props.children.clone()
                    }else{
                    html!{}
                }
                }
            </div>
        </div>
    }
}


