use crate::{GenericNode, View};

macro_rules! define_placeholder {
    ($vis:vis $name:ident($data:expr)) => {
        #[derive(Clone)]
        $vis struct $name<N> {
            node: N,
        }

        impl<N: crate::GenericNode> crate::GenericElement<N> for $name<N> {
            const TYPE: crate::NodeType = crate::NodeType::Placeholder($data);

            fn create_with_node(_: crate::Scope, node: N) -> Self {
                Self { node }
            }

            fn into_node(self) -> N {
                self.node
            }
        }
    };
}

/// 辅助特性，忽略掉当一个视图被卸载时的节点操作。
pub trait ViewParentExt<N: GenericNode> {
    fn as_ref(&self) -> Option<&N>;

    fn append_child(&self, new_view: &View<N>) {
        if let Some(parent) = self.as_ref() {
            new_view.remove_from(parent);
        }
    }

    fn replace_child(&self, new_view: &View<N>, position: &View<N>) {
        if let Some(parent) = self.as_ref() {
            position.replace_with(parent, new_view);
        }
    }

    fn remove_child(&self, position: &View<N>) {
        if let Some(parent) = self.as_ref() {
            position.remove_from(parent);
        }
    }

    fn insert_before(&self, new_view: &View<N>, position: Option<&N>) {
        if let Some(parent) = self.as_ref() {
            new_view.move_before(parent, position);
        }
    }
}

impl<N: GenericNode> ViewParentExt<N> for Option<N> {
    fn as_ref(&self) -> Option<&N> {
        Option::as_ref(self)
    }
}
