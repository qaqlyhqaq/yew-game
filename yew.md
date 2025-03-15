在 Yew 框架中，`html!` 宏允许开发者直接使用所有标准的 **HTML 元素** 作为组件，无需额外导入或封装。这些元素会被 Yew 自动映射到 Rust 代码中，并支持完整的属性、事件绑定和动态内容渲染。以下是核心支持的组件分类及用法详解：


---


### **1. 所有标准 HTML 元素**
Yew 的 `html!` 宏原生支持所有 HTML5 标签，包括但不限于：


#### **常用标签示例**
```rust
html! {
    // 基础结构
    <div id="root">
        // 标题
        <h1>{ "Hello Yew!" }</h1>


        // 段落与文本
        <p class="description">
            { "This is a paragraph with dynamic content: " }{ some_variable }
        </p>


        // 列表
        <ul>
            { for items.iter().map(|item| html! { <li>{ item }</li> }) }
        </ul>


        // 链接
        <a href="https://yew.rs" target="_blank">{ "Yew Documentation" }</a>
    </div>
}
```


#### **表单元素**
```rust
html! {
    <form onsubmit={onsubmit_callback}>
        // 输入框
        <input
            type="text"
            value={input_value}
            oninput={oninput_callback}
            placeholder="Enter text..."
        />


        // 下拉选择
        <select onchange={onchange_callback}>
            <option value="1">{ "Option 1" }</option>
            <option value="2">{ "Option 2" }</option>
        </select>


        // 多选框
        <input type="checkbox" checked={is_checked} onclick={toggle_callback} />


        // 提交按钮
        <button type="submit">{ "Submit" }</button>
    </form>
}
```


#### **事件绑定**
所有元素支持通过 `on*` 属性绑定事件（如 `onclick`, `onchange`）：
```rust
use yew::prelude::*;


#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <button {onclick}>{ "Count: " }{ *counter }</button>
    }
}
```


---


### **2. 路由组件 (`yew-router`)**
虽然 `yew-router` 是一个独立库，但它是 Yew 生态的官方推荐路由解决方案。需在 `Cargo.toml` 中添加依赖：
```toml
[dependencies]
yew-router = "0.17.0"
```


#### **核心路由组件**
- **`BrowserRouter`**: 包裹整个应用，启用基于浏览器 History API 的路由。
- **`Switch`**: 根据当前 URL 路径渲染匹配的组件。
- **`Link`**: 生成导航链接，避免页面刷新。


```rust
use yew::prelude::*;
use yew_router::prelude::*;


// 定义路由枚举
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}


// 实现路由切换逻辑
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}


// 在根组件中使用路由
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            // 导航菜单
            <nav>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
            </nav>


            // 路由内容切换
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
```


---


### **3. 上下文提供者与消费者 (`ContextProvider`)**
用于跨组件层级共享状态，无需逐层传递 Props。


#### **定义与使用**
```rust
use yew::prelude::*;
use yew::context::ContextProvider;


// 定义共享状态结构体
#[derive(Clone, PartialEq)]
struct AppState {
    theme: String,
}


// 根组件提供状态
#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| AppState { theme: "light".to_string() });


    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
            <ChildComponent />
        </ContextProvider<AppState>>
    }
}


// 子组件消费状态
#[function_component(ChildComponent)]
fn child() -> Html {
    let context = use_context::<AppState>().expect("Context not found");


    html! {
        <div class={format!("theme-{}", context.theme)}>
            { "Current theme: " }{ &context.theme }
        </div>
    }
}
```


---


### **4. 动态内容渲染**
在 `html!` 宏中支持多种动态渲染方式：
- **文本插值**: `{variable}`
- **条件渲染**: `if ... else` 或 `match`
- **列表渲染**: `{ for iter.map(...) }`


```rust
html! {
    <div>
        // 条件渲染
        { if is_loading {
            html! { <Spinner /> }
        } else {
            html! { <Content data={data} /> }
        }}


        // 列表渲染
        <ul>
            { for items.iter().map(|item| html! { <li key={item.id}>{ &item.name }</li> }) }
        </ul>
    </div>
}
```


---


### **总结**
Yew 的 `html!` 宏直接支持所有 **标准 HTML 元素** 和 **yew-router 的路由组件**，结合 `ContextProvider` 实现状态共享。开发者无需手动封装即可使用这些核心组件，快速构建动态 Web 应用。对于更复杂的 UI（如表格分页、弹窗），可基于这些基础元素组合自定义组件，或集成社区库（如 `yewprint`）。