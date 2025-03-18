use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use yew::{function_component, html, use_context, use_reducer, use_reducer_eq, AttrValue, Callback, Component, Context, ContextProvider, Html, Properties, Reducible, UseReducerHandle};

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
    html! {
        <>
        <Producer/>
            <span>{format!("theme:{}->{}",msg_ctx.theme,msg_ctx.atomic_count)}</span>
        </>
    }
}




#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    html! {
        <button onclick={move |_| msg_ctx.dispatch(
            AppState{
                theme: "点击计数".to_string(),
                atomic_count:msg_ctx.atomic_count+1
            }
        )}>
            {"PRESS ME"}
        </button>
    }
}
