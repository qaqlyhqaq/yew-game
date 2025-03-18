mod callbackTool;

use crate::structure_plural_function;
use derivative::Derivative;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{
    AttrValue, Callback, Classes, Component, Context, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle, function_component, html, use_context, use_reducer, use_reducer_eq,
};

/// 表达一个导出module 容器,根据获取父组件id,来记录组件
///  大小,夫组件id,以及其他详细内容
pub struct Container {
    //构造大小字段
    size: (u32, u32),
    //父容器id
    parent_id: String,
}

pub enum ActiveWrapper {
    //按钮点击操作
    ClickButton(String),
    //总计数面板更新操作
    TotalCountStateUpdate(HashMap<String, usize>),
}

/// 实现一个从 父容器id 字符串构造 容器方法
impl From<String> for Container {
    fn from(item: String) -> Container {
        Container {
            size: (0, 0),
            parent_id: item,
        }
    }
}

// 定义共享状态结构体
#[derive(Clone, PartialEq, Debug, Eq, Derivative)]
#[derivative(Default)]
pub struct AppState {
    #[derivative(Default(value = r#""我是主题字段!".to_string()"#))]
    pub theme: String,
    #[derivative(Default(value = "0"))]
    atomic_count: isize,
    pub hash_map: HashMap<String, usize>,
}

impl Reducible for AppState {
    type Action = ActiveWrapper;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = AppState {
            theme: self.theme.clone(),
            atomic_count: self.atomic_count,
            hash_map: self.hash_map.clone(),
        };
        match action {
            ActiveWrapper::ClickButton(name) => {
                state.atomic_count += 1;
                state.theme = name;
            }
            ActiveWrapper::TotalCountStateUpdate(map) => {
                state.hash_map = state.hash_map.into_iter().chain(map.into_iter()).collect();
            }
        }
        state.into()
    }
}

pub type MessageContext = UseReducerHandle<AppState>;

#[function_component]
pub fn ContainerLyh() -> Html {
    let msg_ctx = use_reducer::<AppState, _>(|| AppState::default());

    html! {
        <ContextProvider<MessageContext> context={msg_ctx}>
            <Children/>
        </ContextProvider<MessageContext>>
    }
}

#[function_component]
pub fn Children() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let child = vec![
        "全部任务",
        "进行中",
        "已完成",
        "已超期",
        "待开始",
        "暂停",
        "取消",
    ];

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
pub struct HeadItem {
    title: String,
    #[prop_or_default]
    value: Option<String>,
    #[prop_or_default]
    size: Option<(u32, u32)>,
}

#[function_component]
pub fn Producer(head_item: &HeadItem) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    structure_plural_function!(
        "全部任务",
        "进行中",
        "已完成",
        "已超期",
        "待开始",
        "暂停",
        "取消",
        msg_ctx,
        head_item
    )
}
