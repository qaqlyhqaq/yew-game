/*
定义任务管理 trait
 */
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;
use gloo_net::http::Request;
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

impl TaskClient {
    pub(crate) async fn fetch_total_statues(&self) -> BTreeMap<String, usize> {
        log::log!(log::Level::Info, "fetching...");
        let total_url = format!("{}/project/phase/task/numbers", TaskClient::SERVICE_ADDRESS_PREFIX);
        let token = self.token.take().unwrap();
        let response = Request::get(total_url.as_str())
            .header("Authorization", token.as_str())
            .send()
            .await
            .unwrap();

        let value = response
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let token_string = value
            .as_object()
            .unwrap();
        let x = token_string
            .iter().filter(|(k, v)| {
            v.is_i64()
        }).map(|(k, v)| {
            (k.to_string(), v.as_i64().unwrap() as usize)
        })
            .collect();
        x
    }
}

impl ClientBase for TaskClient {
    // const TOKEN: &'static Arc<RefCell<Option<String>>> = &Arc::new(RefCell::new(None));
}

impl TaskManage for TaskClient {
}