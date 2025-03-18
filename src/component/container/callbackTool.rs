/*
回调帮助宏,构建各种回调帮助工具
 */

use yew::{function_component, html};

#[allow(unused)]
#[macro_export]
macro_rules!  structure_plural_function{
    ($($name:literal),*,$tail:ident,$msg_ctx:ident,$head_item:ident) => {
        $(
        if $tail.eq($name){

            let mut style = String::from("padding-left: 1px;");
                style.push_str("padding-right: 20px;");
                style.push_str("margin-left: 20px;");

            let title_count_str = $msg_ctx
                .hash_map
                .get(&$head_item.title)
                .map_or_else(|| "0".to_string(), |count| format!("{}", count));

            html! {
        <button  style={style}  onclick={move  |_:yew::html::onclick::Event|
            {
                let active =  {
                    ActiveWrapper::ClickButton($name.to_string())
                };
                $msg_ctx.dispatch(active)
            }
        }>
            {
                format!("{}({})", $head_item.title.clone() , title_count_str.clone() )
            }
        </button>
    }
        } else )*
         {
             //结尾匹配分支
             unreachable!("出现未匹配值:{}",$tail)
        }
    }
}

#[test]
fn test() {
    let tail = String::from("1");
    // structure_plural_function!("1","2","3","4",tail);
}
