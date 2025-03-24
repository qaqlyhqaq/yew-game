mod callback_tool;
mod vertical_div;
mod collapsible;
mod tr_component;

use crate::component::container::collapsible::Collapsible;
use crate::component::container::tr_component::TrConponent;
use crate::component::container::vertical_div::VerticalDiv;
use crate::structure_plural_function;
use derivative::Derivative;
use gloo::utils::document;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::Element;
use yew::{function_component, html, use_context, use_reducer, Callback, Component, ContextProvider, Html, NodeRef, Properties, Reducible, UseReducerHandle};
use crate::net::task_manage::TaskClient;
// use yew::format::Nothing;
// use yew::services::fetch::Request;

pub enum ActiveWrapper {
    //按钮点击操作
    ClickButton(String),
    //总计数面板更新操作
    TotalCountStateUpdate(HashMap<String, usize>),
}

#[derive(Clone, PartialEq, Debug, Eq, Derivative)]
#[derivative(Default)]
struct ButtonState {
    //最后点击完成状态按钮事件
    #[derivative(Default(value = "None"))]
    last_click_compile_button: Option<String>,
    //最后点击,任务分类按钮状态
    #[derivative(Default(value = "None"))]
    last_click_task_classify_button: Option<String>,
    //最后修改分页状态
    #[derivative(Default(value = "None"))]
    last_page_state: Option<u32>,
}

// 定义共享状态结构体
#[derive(Clone, PartialEq, Debug, Eq, Derivative)]
#[derivative(Default)]
pub struct AppState {
    //按钮被点击次数字段
    #[derivative(Default(value = "0"))]
    atomic_count: isize,
    //按钮属性字段
    pub hash_map: HashMap<String, usize>,
    //默认外部选项按钮字段结构
    pub  button_state:ButtonState,
}

impl Reducible for AppState {
    type Action = ActiveWrapper;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = AppState {
            atomic_count: self.atomic_count,
            hash_map: self.hash_map.clone(),
            button_state: self.button_state.clone(),
        };
        match action {
            ActiveWrapper::ClickButton(name) => {
                state.atomic_count += 1;
                //重置最后点击任务分类按钮状态
                state.button_state.last_click_task_classify_button = Some(name);
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
    pub table_body_ref: NodeRef,
    #[prop_or_default]
    pub client:TaskClient,
}

static   vertical_div_items: [&str; 4] = ["全部任务", "我创建的任务", "我参与的任务", "下属的任务"];
static table_header: [&str; 10] = ["任务编号","任务名称","优先级","所属项目","项目阶段","负责人","起止时间","任务进度","状态","操作"];


/// 内容提供着,为页面提供点击的任务状态,
/// 以及任务所有着状态信息!
#[function_component(MainContainer)]
pub fn container_component(prop:&ContainerProperties) -> Html {
    let msg_ctx = use_reducer::<AppState, _>(|| AppState::default());

    let mut client = prop.client.clone();

    if client.token.get.is_none() {
        
    }

    let node_ref = prop.table_body_ref.clone();

    html! {
        <ContextProvider<MessageContext> context={msg_ctx}>
            <Children/>
        <div  style="display: flex; row: column;"  >
            <VerticalDiv >
                <span>{"任务分类"}</span>
                {
                    vertical_div_items
                        .iter()
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
                        {table_header.clone()
                            .iter()
                            .map(|x|{html!{
                            <th style="border: 1px solid #999; padding: 8px;" >{x}</th>
                        }})
                            .collect::<Html>()
                        }
                        </tr>
                    </thead>
                    <tbody  ref={prop.table_body_ref.clone()}  >
                    </tbody>
                </table>
                //下分页配置选项
                <div  style="margin-left:200px;display: grid; grid-template-columns: 1fr 2fr 5fr;">
                    <span>{"共10页/100条数据"}</span>
                    //定义 ref 标签,后续进行自定义渲染分页标签列表到此
                    <div  ></div>
                    <div>
                        <span>{"跳至"}</span>
                        <input value={1} type="number" />
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
        <div style="display: block;background-color: #E9967A;" >
        {for child.into_iter()
        .map(|item|html!{
            <Producer title={item} size={(80,30)} />
        })}
        </div>
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
