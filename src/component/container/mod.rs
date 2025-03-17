use std::rc::Rc;
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
}


impl Reducible for AppState {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppState { theme: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<AppState>;


impl Component for Container {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let container = Container::from("默认窗口字段:".to_string());
        container
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // let msg_ctx = use_reducer::<AppState, _>(|| {
        //
        // });

        html! {
            <ContextProvider<AppState> context={AppState::default()}>
                <Children/>
            </ContextProvider<AppState>>
        }
        // html! {
        //     <ContextProvider<MessageContext> context={msg}>
        //         <Producer/>
        //         <br/>
        //         <span>{self.parent_id.clone()}</span>
        //         <br/>
        //         <Children/>
        //     </ContextProvider<MessageContext>>
        // }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
    }
}


#[function_component]
pub fn Children() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
        // .and_then(||{Some(use_effect_with_deps())});


    html! {
        <>
            <span>{format!("theme:{}",msg_ctx.theme)}</span>
        </>
    }
}




#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    html! {
        <button onclick={move |_| msg_ctx.dispatch("Message Received.".to_string())}>
            {"PRESS ME"}
        </button>
    }
}
