#![allow(clippy::type_complexity)]

#[doc(hidden)]
#[path = "private.rs"]
pub mod __private;

#[macro_use]
mod macros;
#[macro_use]
mod utils;
mod component;
mod element;
mod node;
mod reactive;
mod view;

pub mod components {
    mod element;
    mod fragment;

    pub use element::Element;
    pub use fragment::Fragment;
}
pub mod elements;
pub mod template;

#[doc(inline)]
pub use {
    component::{DynComponent, GenericComponent},
    components::*,
    element::GenericElement,
    node::{DomNode, EventHandler, GenericNode, NodeType},
    reactive::{create_root, untrack, Effect, Scope, ScopeDisposer, Signal},
    view::View,
};

/// 创建一个元素组件 [`struct@Element`]。
pub fn view<N: GenericNode>(cx: Scope) -> Element<N> {
    Element(cx)
}
