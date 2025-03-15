use yew::ToHtml;

/// 抽象 item 设计,横向布局
#[derive(Debug)]
pub struct Item<T>
where
T: ToHtml
{
    //T 为一个属性,可自由转换到某个项
    props: Vec<T>,
}