


/// 表达一个导出module 容器,根据获取父组件id,来记录组件
///  大小,夫组件id,以及其他详细内容
pub struct Container {
    //构造大小字段
    size:(u32, u32),
    //父容器id
    parent_id:String,
}

/// 实现一个从 父容器id 字符串构造 容器方法
impl  From<String> for Container {
    fn from(item:String) -> Container {
        Container{
            size:(0,0),
            parent_id:item,
        }
    }
}