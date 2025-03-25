/*
回调帮助宏,构建各种回调帮助工具
 */

use yew::{function_component, html};

//辅助 tt 下坠,帮助构造点击方法,避免本地变量声明周期逃离问题
#[allow(unused)]
#[macro_export]
macro_rules!  structure_plural_function{
    ($($name:literal),*,$msg_ctx:ident,$properties:ident) => {
        $(
        if $properties.title.clone().eq($name){
            let mut style = String::from("padding-left: 1px;");
                style.push_str("padding-right: 20px;");
                style.push_str("margin-left: 20px;margin-top: 15px;margin-bottom: 13px;");
            let _ = $properties.size.is_some_and(|t| {
                style.push_str(format!("height:{}px;", t.1).as_str());
                true
            });

            let title_count_str = $properties.value.clone().map_or_else(|| "0".to_string(), |count| format!("{}", count));

            html! {
            <button  style={style}  onclick={move  |_:yew::html::onclick::Event|
            {
                //处理按钮点击分发事件
                let active =  {
                    ActiveWrapper::ClickButton($name.to_string())
                };
                $msg_ctx.dispatch(active)
            }
            }>
            {
                format!("{}({})", $properties.title.clone() , title_count_str.clone() )
            }
            </button>
            }
            } else )*
         {
             //结尾匹配分支
             unreachable!("出现未匹配值:{}",$properties.title.clone())
        }
    }
}


