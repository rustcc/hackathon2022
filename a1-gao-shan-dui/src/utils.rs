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
    fn with_parent(&self, f: impl FnOnce(&N));

    fn append_child(&self, new_view: &View<N>) {
        self.with_parent(|parent| {
            new_view.remove_from(parent);
        });
    }

    fn replace_child(&self, new_view: &View<N>, old_view: &View<N>) {
        self.with_parent(|parent| {
            old_view.replace_with(parent, new_view);
        });
    }

    fn remove_child(&self, position: &View<N>) {
        self.with_parent(|parent| {
            position.remove_from(parent);
        });
    }

    fn insert_before(&self, new_view: &View<N>, position: Option<&N>) {
        self.with_parent(|parent| {
            new_view.move_before(parent, position);
        });
    }
}

impl<N: GenericNode> ViewParentExt<N> for N {
    fn with_parent(&self, f: impl FnOnce(&N)) {
        f(self);
    }
}

impl<N: GenericNode> ViewParentExt<N> for Option<N> {
    fn with_parent(&self, f: impl FnOnce(&N)) {
        self.as_ref().with_parent(f);
    }
}

impl<N: GenericNode> ViewParentExt<N> for Option<&N> {
    fn with_parent(&self, f: impl FnOnce(&N)) {
        self.map(|n| n.with_parent(f));
    }
}
