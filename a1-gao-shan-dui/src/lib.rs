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
mod event_delegation;
mod node;
mod reactive;
mod view;

pub mod components {
    mod element;
    #[path = "for.rs"]
    mod for_;
    mod fragment;
    mod list;
    mod show;

    pub use element::Element;
    pub use for_::For;
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
    element::{GenericElement, NodeRef},
    elements::{IntoEventHandler, IntoReactive, Reactive, Reactive::Value},
    node::{mount_to, mount_to_body, DomNode, EventHandler, GenericNode, NodeType, Property},
    reactive::{create_root, untrack, Effect, Scope, ScopeDisposer, Signal},
    view::View,
    web_sys::Event,
};

use std::borrow::Borrow;

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap();
    static DOCUMENT: web_sys::Document = WINDOW.with(web_sys::Window::document).unwrap();
}

/// 创建一个元素组件 [`struct@Element`]。
pub fn view<N: GenericNode>(cx: Scope) -> Element<N> {
    Element(cx)
}

pub trait ScopeExt: Borrow<Scope> {
    fn create_node_ref<N: GenericNode>(&self) -> NodeRef<N> {
        NodeRef::new(*self.borrow())
    }
}

impl<T: Borrow<Scope>> ScopeExt for T {}
