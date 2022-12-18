#![allow(clippy::type_complexity)]

mod component;
mod element;
mod node;
mod reactive;
mod view;

pub mod template;
pub mod components {
    mod element;
    mod fragment;

    pub use element::Element;
    pub use fragment::Fragment;
}

#[doc(inline)]
pub use {
    component::{DynComponent, GenericComponent},
    components::*,
    element::GenericElement,
    node::{EventHandler, GenericNode, NodeType},
    reactive::{create_root, untrack, Effect, Scope, ScopeDisposer, Signal},
    view::View,
};
