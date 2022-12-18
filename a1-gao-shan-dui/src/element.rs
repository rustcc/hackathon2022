use crate::{GenericNode, NodeType, Scope};

/// 某个节点类型所对应的元素，此接口主要用来创建 [`Element`] 组件。
///
/// [`Element`]: struct@crate::components::Element
pub trait GenericElement<N: GenericNode>: 'static + Sized {
    /// 该元素对应的 [`NodeType`]。
    const TYPE: NodeType;

    /// 从给定节点创建一个元素。
    fn create_with_node(cx: Scope, node: N) -> Self;

    /// 返回该元素对应的节点。
    fn into_node(self) -> N;

    /// 创建一个新元素。
    fn create(cx: Scope) -> Self {
        Self::create_with_node(cx, N::create(Self::TYPE))
    }
}
