use std::collections::HashMap;
use std::fmt::format;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use yew::{function_component, html, use_context, use_reducer, use_reducer_eq, AttrValue, Callback, Classes, Component, Context, ContextProvider, Html, Properties, Reducible, UseReducerHandle};

use derivative::Derivative;

/// 表达一个导出module 容器,根据获取父组件id,来记录组件
///  大小,夫组件id,以及其他详细内容
pub struct Container {
    //构造大小字段
    size:(u32, u32),
    //父容器id
    parent_id:String,
}

/// 实现一个从 父容器id 字符串构造 容器方法
impl  From<String> for Container {
    fn from(item:String) -> Container {
        Container{
            size:(0,0),
            parent_id:item,
        }
    }
}





// 定义共享状态结构体
#[derive(Clone, PartialEq)]
#[derive(Debug, Eq)]
#[derive(Derivative)]
#[derivative(Default)]
pub struct AppState {
    #[derivative(Default(value = r#""我是主题字段!".to_string()"#))]
    pub theme: String,
    #[derivative(Default(value = "0"))]
    atomic_count:isize,
}


impl Reducible for AppState {
    type Action = AppState;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}

pub type MessageContext = UseReducerHandle<AppState>;


#[function_component]
pub fn ContainerLyh() -> Html{
        let msg_ctx = use_reducer::<AppState, _>(|| {
            AppState::default()
        });

        html! {
            <ContextProvider<MessageContext> context={msg_ctx}>
                <Children/>
            </ContextProvider<MessageContext>>
        }
}


#[function_component]
pub fn Children() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let child = vec!["全部任务","进行中","已完成","已超期","待开始","暂停","取消"];

    html! {
        <>
        <>
        {for child.into_iter()
        .map(|item|html!{
            <Producer title={item} size={(80,30)} />
        })}
        </>
            <br/>
            <span>{
            if msg_ctx.atomic_count == AppState::default().atomic_count{
                "未开始点击".to_string()
            }else{
            format!("内容:{}->{}",msg_ctx.theme,msg_ctx.atomic_count)
            }
        }</span>
        </>
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct HeadItem{
    title:String,
    #[prop_or_default]
    value:Option<String>,
    #[prop_or_default]
    size:Option<(u32,u32)>,
}


#[function_component]
pub fn Producer(head_item: &HeadItem) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let title = format!("last click : {}", head_item.title.clone());

    let mut style= String::new() ;

    let _ = head_item.size
        .is_some_and(|t| {
            style = format!("width:{}px;height:{}px;", t.0, t.1);
            true
        });

    html! {
        <button  style={style}  onclick={move |_| msg_ctx.dispatch(
            AppState{
                theme: title.clone(),
                atomic_count:msg_ctx.atomic_count+1
            }
        )}>
            {
                format!("{}",head_item.title.clone())
            }
        </button>
    }
}
