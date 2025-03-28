/*
零时请求模块,零时编写的请求一律放此处
 */
use std::cell::RefCell;
use std::sync::Arc;
use gloo_net::http::Request;
use serde_json::{json, Map};
use yew::Properties;


pub trait ClientBase{
    //公共服务器地址前缀
    const SERVICE_ADDRESS_PREFIX: &'static str = "http://10.60.1.240:8081";
    //客户端token
    // const TOKEN:&'static Arc<RefCell<Option<String>>>;

    /// trait 级别方法,
    /// 用于登录失效情况
    async fn login() -> String{
        let value1 = json!(
            {
  "code": "",
  "password": "admin123",
  "status": 0,
  "username": "admin",
  "uuid": ""
}
        );
        let login_url = format!("{}/login", Self::SERVICE_ADDRESS_PREFIX);
        let response = Request::post(login_url.as_str())
            .header("Content-Type", "application/json")
            .body(value1.to_string())
            .unwrap()
            .send()
            .await
            .unwrap();
        let value = response
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let token_string = value
            .as_object()
            .unwrap()
            .get("token")
            .unwrap().as_str()
            .unwrap().to_string();

        token_string

    }
}


/// 客户端请求实现集合,
/// 自动权健token
#[derive(Properties, Debug, PartialEq,Default)]
struct RequestClient{
    //token 请求句柄
    #[prop_or_default]
    token:Option<String>,
}