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
pub trait ViewParentExt<'a, N: GenericNode>: Into<Option<&'a N>> {
    fn append_child(self, new_view: &View<N>) {
        if let Some(parent) = self.into() {
            new_view.remove_from(parent);
        }
    }

    fn replace_child(self, new_view: &View<N>, position: &View<N>) {
        if let Some(parent) = self.into() {
            position.replace_with(parent, new_view);
        }
    }

    fn remove_child(self, position: &View<N>) {
        if let Some(parent) = self.into() {
            position.remove_from(parent);
        }
    }

    fn insert_before(self, new_view: &View<N>, position: Option<&N>) {
        if let Some(parent) = self.into() {
            new_view.move_before(parent, position);
        }
    }

    /// 遍历全部节点，检查是否与 `first` 及其之后的兄弟节点顺序一致。
    fn check_children(self, view: &View<N>) -> bool {
        let mut equal = true;
        if self.into().is_some() {
            let mut current = Some(view.first());
            view.visit(|node| {
                if let Some(real) = current.as_ref() {
                    if real.ne(node) {
                        equal = false;
                    }
                    current = real.next_sibling();
                } else {
                    equal = false;
                }
            });
        }
        equal
    }
}

impl<'a, N: GenericNode, T: Into<Option<&'a N>>> ViewParentExt<'a, N> for T {}
