/*
定义任务管理 trait
 */
use std::cell::RefCell;
use std::sync::Arc;
use crate::net::request::ClientBase;

/// 任务管理客户端实现请求接口
trait TaskManage : ClientBase{
}

pub struct TaskClient {
}

impl ClientBase for TaskClient {
    // const TOKEN: &'static Arc<RefCell<Option<String>>> = &Arc::new(RefCell::new(None));
}

impl TaskManage for TaskClient {
}