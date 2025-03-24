/*
零时请求模块,零时编写的请求一律放此处
 */
use yew::Properties;

/// 客户端请求实现集合,
/// 自动权健token
#[derive(Properties, Debug, PartialEq,Default)]
struct RequestClient{
    //token 请求句柄
    #[prop_or_default]
    token:Option<String>,
}