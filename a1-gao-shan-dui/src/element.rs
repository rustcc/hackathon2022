use std::any::Any;

use crate::{GenericNode, NodeType, Scope, Signal};

/// 某个节点类型所对应的元素，此接口主要用来创建 [`Element`] 组件。
///
/// [`Element`]: struct@crate::components::Element
pub trait GenericElement<N: GenericNode>: 'static + Clone {
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

    fn ref_(self, ref_: NodeRef<N>) -> Self {
        ref_.inner.set(Some(self.clone().into_node()));
        self
    }
}

/// 指向某个节点的引用。
pub struct NodeRef<N: 'static> {
    inner: Signal<Option<N>>,
}

impl<N> Clone for NodeRef<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N> Copy for NodeRef<N> {}

impl<N: GenericNode> NodeRef<N> {
    pub(crate) fn new(cx: Scope) -> Self {
        Self {
            inner: cx.create_signal(None),
        }
    }

    /// 读取当前引用被转换成特定的 [`GenericNode`] 实现。
    ///
    /// # Panic
    ///
    /// 如果当前引用尚未被绑定则会导致 `panic`。
    pub fn get<U: GenericNode>(&self) -> Option<U> {
        let node = self.inner.get().expect("`NodeRef` 尚未被绑定");
        (&node as &dyn Any).downcast_ref::<U>().cloned()
    }
}
