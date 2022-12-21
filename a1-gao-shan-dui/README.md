# aKun

> A1 高闪队，希望能让您“眼前一亮”

细粒度更新、声明式 Web 框架的简单实现。

## ✨ 核心特性

- 类似 [Solid.js](https://www.solidjs.com/) 的响应系统
- 在运行时为任意组件创建模板
- 由组件管理的 UI 结构，最大限度掌控渲染的行为
- 基于 Builder Pattern，宏是可选的

## ✍️ 示例

```rust
akun::mount_to_body(|cx| {
    let counter = cx.create_signal(0);
    let increment = move |_| counter.update(|x| *x + 1);
    akun::view! { cx,
        div {
            button {
                @click(increment)
                "Click me: " (counter) " times!"
            }
        }
    }
});
```

更多示例请查看 [examples](examples)

## 💭 实现细节

`akun` 很大程度上是我们另一个项目 [xfame](https://github.com/loichyan/xframe) 的简化实现，主要由 3 个部分组成：

- `reactive`: 响应系统
- `node`: 修改节点树的接口
- `component`: 渲染 UI 视图的核心特性

### `akun::reactive`

这是 `akun` 实现响应性更新的核心，`akun::reactive` 实现了类似
[Solid.js](https://www.solidjs.com/)、[Sycamore](https://sycamore-rs.netlify.app/)、[Leptos](https://github.com/gbj/leptos)
等框架提供的响应系统。

考虑下面这个示例，

```rust
// 通过 create_root 我们得以引入响应系统
create_root(|cx| {
    // 1) 我们首先创建一个 Signal (信号)，
    let state = cx.create_signal(1);
    // 2) 然后创建一个 Effect (副作用)，
    cx.create_effect(move || {
        // 3) 随即 Effect 被调用，然后它发现 state 在其内部被访问了，
        // 此时 Effect 会将 state 记录下来，并且订阅它的下一次更新
        let state = state.get();
        println!("state = {}", state);
    });
    // ...
    // 4) 之后 state 被设置了一个新值，然后它发现上述的 Effect 订阅了
    // 它的这次更新，故它会通知 Effect 作出反应，我们又回到了 (3)
    state.set(2);
});
```

上述便是这个响应系统的基本原理，具体的实现细节上，我们维护了一个全局的观察者（`observer`），每当
`Effect` 被运行时，它就会将自己设为 `observer`，而当 `Signal` 被读取时，它会检查当前的 `ovserver`
并记录下来，之后的下一次更新 `Signal` 会通知所有被记录的 `Effect` 作出反应。

### `akun::node`

`GenericNode` 是 `akun` 呈现 UI 的核心接口，它定义了一系列用于修改节点属性以及节点树的方法，
`DomNode` 是面向 Web 的实现。`GenericNode` 是用户界面的最小单元，`GenericElement`
是针对不同的节点类型的包装，每个 `GenericElement` 的实现对应一种节点，常见的有带有标签的 HTML
元素、文本节点、占位符（`Comment` 节点）等。

视图（`View`）是某个组件挂载到节点树上全部内容的镜像，它可能是一个单独的节点（`ViewType::Node`）、
一个片段（`ViewType::Fragment`）或者一个动态更新的视图（`ViewType::Dyn`），节点对应了页面中的某个元素，片段则表示一系列相邻的节点树。

### `akun::component`

`GenenicComponent` 是 `akun` 实现动态创建模板以及细粒度更新视图的核心特性，它强迫用户在创建 UI
界面时分两个阶段，初始化（`init`）和渲染（`render`）。

在初始化阶段，`init` 返回一个节点树，表示某个组件最原始的状态，即没有设定任何属性或者监听某些事件等。
组件应该保证初始化阶段返回的视图永远都是一致的，否则会导致渲染阶段陷入异常。对于“静态”组件，
即元素（`Element`）和片段(`Fragment`)，他们分别返回 `ViewType::Node` 和
`ViewType::Fragment`；而对于“动态”组件，最常见的做法是将其视为一个特殊的 `ViewType::Node`，
即在初始化阶段返回一个占位符用来标定该组件的位置，随后在渲染阶段将其替换成实际的视图。

在渲染阶段，`render` 接受其 `init` 返回的视图中的第一个节点 <sup>1</sup> 作为参数，返回其之后的第一个兄弟节点
<sup>2</sup>，以及渲染后的视图。对于“静态”组件，返回的视图应该与 `init` 返回的保持一致，而“动态”组件，则应该返回满足如下条件的动态视图：

- 此视图不允许为空
- 每次读取此视图时总能得到该组件挂载到节点树上的全部节点，且顺序一致

基于上述条件，一个组件动态更新视图时，可以将其挂载的子组件视为一个整体，而不需要遍历其全部的节点，来与当前视图作 `diff`，从而减少了一定的开销。

- [1] 实际上返回的是缓存在全局模板中的一个副本
- [2] 这样设计的意义是为了使得多个组件能“管道”连接，即前一个组件返回的下一个节点是后一个组件的第一个节点

## 💻 开发环境

本项目使用 [nixpkgs](https://github.com/NixOS/nixpkgs) 以及 [direnv](https://direnv.net/)
部署环境，基于 `rustc v1.66` 开发，你可以通过：

```sh
direnv allow
# 或者
nix develop
```

来搭建相同的开发环境。

## 📝 待办

- [x] `For` 组件
- [ ] 全局事件托管

## ⚖️ 许可

根据以下任意一种许可发布：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
