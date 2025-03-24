/*
定义任务管理 trait
 */
use std::cell::RefCell;
use std::sync::Arc;
use crate::net::request::ClientBase;

/// 任务管理客户端实现请求接口
trait TaskManage : ClientBase{
    const TOKEN: &'static Arc<RefCell<Option<String>>>;
}

pub struct TaskClient {
    pub(crate) token:  Arc<RefCell<Option<String>>> ,
}

impl ClientBase for TaskClient {
    const TOKEN: &'static Arc<RefCell<Option<String>>> = Self::TOKEN;
}

impl TaskManage for TaskClient {
    const TOKEN: &'static Arc<RefCell<Option<String>>> = &Self.token;
}