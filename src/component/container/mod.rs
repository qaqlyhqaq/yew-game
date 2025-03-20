mod callbackTool;
mod verticalDiv;
mod collapsible;
mod trConponent;

use crate::component::container::verticalDiv::VerticalDiv;
use crate::structure_plural_function;
use derivative::Derivative;
use std::collections::HashMap;
use std::rc::Rc;
use gloo::timers::callback::Interval;
use gloo::utils::document;
use web_sys::Element;
use yew::{function_component, html, use_context, use_reducer, Callback, Component, ContextProvider, Html, NodeRef, Properties, Reducible, UseReducerHandle};
use crate::component::container::collapsible::Collapsible;
use crate::component::container::trConponent::{TrConponent, TrProperties};

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

#[derive(Properties, Debug, PartialEq)]
pub struct ContainerProperties {
    #[prop_or_default]
    pub t_body_ref: NodeRef,
}

//style="display: flex; flex-direction: column;"
#[function_component(MainContainer)]
pub fn container_component(prop:&ContainerProperties) -> Html {
    let msg_ctx = use_reducer::<AppState, _>(|| AppState::default());

    let vertical_div_items = vec!["全部任务","我创建的任务","我参与的任务","下属的任务"];


    let header_text = vec!["任务编号","任务名称","优先级","所属项目","项目阶段","负责人","起止时间","任务进度","状态","操作"];

    let mut body_vec = vec![];
    for _ in 0..10{
        body_vec.push(header_text.clone())
    }

    let node_ref = prop.t_body_ref.clone();

    html! {
        <ContextProvider<MessageContext> context={msg_ctx}>
            <Children/>
        <div  style="display: flex; row: column;"  >
            <VerticalDiv >
                <span>{"任务分类"}</span>
                {
                    vertical_div_items
                        .into_iter()
                        .map(|item| {html!{<button >{item}</button>}})
                        .collect::<Vec<_>>()
                }
            </VerticalDiv>
         <div  style="display: flex;flex-direction: column;"  >
            <Collapsible button_back_call={Callback::from(move |_|{
                    let app_container = node_ref
                        .cast::<Element>()
                        .expect("对象或许还未挂载!");

                    //此处创建 Table 下 Tr 对象 , 方便子内容进行表格布局合并操作
                    let app_div = document()
                        .create_element("tr")
                        .expect("Failed to create <tr> element");

                    let _ = app_container
                        .append_child(&app_div)
                        .expect("Failed to append app div app container div");

                    let new_counter_app = yew::Renderer::<TrConponent>::with_root_and_props(
                        app_div.clone(),
                        (),
                    )
                    .render();
                    })}  title="展开">
                    <div style="display: grid;grid-template-columns: 1fr 1fr 1fr;">
                    //第一行输入信息栏
                    <div>
                    <label>{"任务名称:"}</label>
                    <input type="text" placeholder="任务编号/任务名称" name="<UNK>" id="<UNK>" checked=true/>
                    </div>
                    <div>
                    <label>{"所属项目:"}</label>
                    <select >
                        <option selected=true>{"选项一"}</option>
                        <option selected=true>{"选项二"}</option>
                        <option selected=true>{"选项三"}</option>
                    </select>
                    </div>
                    <div>
                    <label>{"优先级:"}</label>
                    <select >
                        <option selected=true>{"选项一"}</option>
                        <option selected=true>{"选项二"}</option>
                        <option selected=true>{"选项三"}</option>
                    </select>
                    </div>
                    </div>
                    <br/>
                    //第二行输入信息栏
                    <label>{"负责人:"}</label>
                    <input type="text" placeholder="负责人姓名" name="<UNK>" id="<UNK>" checked=true/>
                    <label>{"状态:"}</label>
                    <select >
                        <option selected=true>{"选项一"}</option>
                        <option selected=true>{"选项二"}</option>
                        <option selected=true>{"选项三"}</option>
                    </select>
            </Collapsible >
            <div style="min-width:600px;">
                <table style="border-collapse: collapse; width: 100%;">
                    <thead style="border: 2px solid #666;" >
                        <tr>
                        {header_text.clone().into_iter()
                            .map(|x|{html!{
                            <th style="border: 1px solid #999; padding: 8px;" >{x}</th>
                        }})
                            .collect::<Html>()
                        }
                        </tr>
                    </thead>
                    <tbody  ref={prop.t_body_ref.clone()}  >
                    </tbody>
                </table>
                //下分页配置选项
                <div  style="margin-left:200px;display: grid; grid-template-columns: 1fr 2fr 5fr;">
                    <span>{"共10页/100条数据"}</span>
                    //定义 ref 标签,后续进行自定义渲染分页标签列表到此
                    <div  ></div>
                    <div>
                        <span>{"跳至"}</span>
                        <input  type="number" />
                        <span>{"页"}</span>
                    </div>
                </div>
            </div>
        </div>
        </div>
        </ContextProvider<MessageContext>>
    }
}

//background:black;

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
