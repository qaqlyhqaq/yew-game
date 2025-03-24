/*
定义任务管理 trait
 */
use std::cell::RefCell;
use std::sync::Arc;
use yew::html::ImplicitClone;
use yew::Properties;
use crate::net::request::ClientBase;

/// 任务管理客户端实现请求接口
trait TaskManage : ClientBase{
}

#[derive(Properties, Debug, PartialEq,Default,Clone)]
pub struct TaskClient {
    #[prop_or_default]
    pub token: Arc<RefCell<Option<String>>>,
}

impl ClientBase for TaskClient {
    // const TOKEN: &'static Arc<RefCell<Option<String>>> = &Arc::new(RefCell::new(None));
}

impl TaskManage for TaskClient {
}