use super::IntoReactive;
use crate::{GenericElement, GenericNode, NodeType, Property, Scope};

/// 文本节点。
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct text<N> {
    cx: Scope,
    node: N,
}

impl<N: GenericNode> text<N> {
    pub fn data<A: IntoReactive<Property>>(self, data: A) -> Self {
        let node = self.node.clone();
        let data = data.into_reactive();
        self.cx
            .create_effect(move || node.set_inner_text(&data.clone().into_value().into_string()));
        text {
            cx: self.cx,
            node: self.node,
        }
    }
}

impl<N: GenericNode> GenericElement<N> for text<N> {
    const TYPE: NodeType = NodeType::Text("");

    fn create_with_node(cx: Scope, node: N) -> Self {
        Self { cx, node }
    }

    fn into_node(self) -> N {
        self.node
    }
}

/// 创建一个文本节点 [`struct@text`]。
pub fn text<N: GenericNode>(cx: Scope) -> text<N> {
    text::create(cx)
}
