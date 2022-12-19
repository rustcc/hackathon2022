#![allow(clippy::type_complexity)]

/// 非公开 API。
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
    mod list;
    mod show;

    pub use element::Element;
    pub use fragment::Fragment;
    pub use list::List;
    pub use show::{Else, If, Show};
}
pub mod elements;
pub mod template;

#[doc(inline)]
pub use {
    component::{DynComponent, GenericComponent},
    components::*,
    element::GenericElement,
    elements::{IntoEventHandler, IntoReactive, Reactive, Reactive::Value},
    node::{mount_to, mount_to_body, DomNode, EventHandler, GenericNode, NodeType, Property},
    reactive::{create_root, untrack, Effect, Scope, ScopeDisposer, Signal},
    view::View,
    web_sys::Event,
};

/// 创建一个元素组件 [`struct@Element`]。
pub fn view<N: GenericNode>(cx: Scope) -> Element<N> {
    Element(cx)
}
