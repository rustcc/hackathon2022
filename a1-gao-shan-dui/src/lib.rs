mod component;
mod element;
mod node;
mod reactive;
mod view;

pub mod template;
pub mod components {
    mod element;

    pub use element::Element;
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
