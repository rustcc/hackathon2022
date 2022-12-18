mod component;
mod node;
mod reactive;
mod view;

pub mod template;

#[doc(inline)]
pub use {
    component::{DynComponent, GenericComponent},
    node::{EventHandler, GenericNode, NodeType},
    reactive::{create_root, untrack, Effect, Scope, ScopeDisposer, Signal},
    view::View,
};
