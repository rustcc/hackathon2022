macro_rules! define_placeholder {
    ($vis:vis $name:ident($data:expr)) => {
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
