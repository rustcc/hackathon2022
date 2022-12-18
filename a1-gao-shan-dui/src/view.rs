use crate::{GenericNode, Signal};
use std::rc::Rc;

type ViewDyn<N> = Rc<dyn Fn() -> View<N>>;

/// 细粒度更新视图的核心机制。每个组件通过 [`View`] 与其父组件同步视图，以保证父
/// 组件可将其视为一个整体来执行动态更新。
#[derive(Clone)]
pub enum View<N> {
    Node(N),
    Fragment(Rc<[View<N>]>),
    Dyn(ViewDyn<N>),
}

impl<N> From<Vec<View<N>>> for View<N> {
    fn from(t: Vec<View<N>>) -> Self {
        Self::Fragment(t.into_boxed_slice().into())
    }
}

impl<N, F> From<F> for View<N>
where
    F: 'static + Fn() -> View<N>,
{
    fn from(f: F) -> Self {
        Self::Dyn(Rc::new(f))
    }
}

impl<N: Clone> From<Signal<View<N>>> for View<N> {
    fn from(t: Signal<View<N>>) -> Self {
        Self::from(move || t.get())
    }
}

impl<N: GenericNode> View<N> {
    /// 对所有的 [`View::Node`] 节点执行 `GenericNo::deep_clone`，对于
    /// [`View::dyn`] 则拷贝其引用。
    pub fn deep_clone(&self) -> View<N> {
        match self {
            Self::Node(t) => Self::Node(t.deep_clone()),
            Self::Fragment(t) => t.iter().map(|t| t.deep_clone()).collect::<Vec<_>>().into(),
            Self::Dyn(t) => Self::Dyn(t.clone()),
        }
    }

    /// 遍历全部节点，[`View::dyn`] 将会被立即执行。
    pub fn visit(&self, mut f: impl FnMut(&N)) {
        self.visit_impl(&mut f);
    }

    /// 使用 `&mut impl ...` 防止递归调用时 `f` 的类型无限嵌套。
    fn visit_impl(&self, f: &mut impl FnMut(&N)) {
        match self {
            Self::Node(t) => f(t),
            Self::Fragment(t) => t.iter().for_each(|t| t.visit_impl(f)),
            Self::Dyn(t) => t().visit_impl(f),
        }
    }

    /// [`visit`] 全部节点并逐个附加至 `parent`。
    pub fn append_to(&self, parent: &N) {
        self.visit(|t| parent.append_child(t));
    }
}
