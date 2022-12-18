use crate::{GenericNode, Signal};
use std::rc::Rc;

type ViewDyn<N> = Rc<dyn Fn() -> View<N>>;

/// 细粒度更新视图的核心机制。每个组件通过 [`View`] 与其父组件同步视图，以保证父
/// 组件可将其视为一个整体来执行动态更新。
#[derive(Clone)]
pub struct View<N>(ViewType<N>);

#[derive(Clone)]
enum ViewType<N> {
    Node(N),
    Fragment(Rc<[View<N>]>),
    Dyn(ViewDyn<N>),
}

impl<N> ViewType<N> {
    fn fragment(views: Vec<View<N>>) -> Self {
        Self::Fragment(views.into_boxed_slice().into())
    }
}

impl<N, F> From<F> for View<N>
where
    F: 'static + Fn() -> View<N>,
{
    fn from(f: F) -> Self {
        Self(ViewType::Dyn(Rc::new(f)))
    }
}

impl<N: Clone> From<Signal<View<N>>> for View<N> {
    fn from(t: Signal<View<N>>) -> Self {
        Self::from(move || t.get())
    }
}

impl<N: GenericNode> View<N> {
    /// 创建一个节点视图。
    pub fn node(node: N) -> Self {
        Self(ViewType::Node(node))
    }

    /// 创建一个片段视图。
    ///
    /// # Panic
    ///
    /// 当给定 `views` 为空时，引发 `panic`。
    pub fn fragment(views: Vec<View<N>>) -> Self {
        if views.is_empty() {
            panic!("`View` 不允许为空")
        }
        Self(ViewType::fragment(views))
    }

    /// 对所有的节点执行 [`deep_clone`]，动态视图则拷贝其引用。
    ///
    /// [`deep_clone`]: [`GenericNode::deep_clone`]
    pub fn deep_clone(&self) -> View<N> {
        Self(match &self.0 {
            ViewType::Node(t) => ViewType::Node(t.deep_clone()),
            ViewType::Fragment(t) => {
                ViewType::fragment(t.iter().map(|t| t.deep_clone()).collect::<Vec<_>>())
            }
            ViewType::Dyn(t) => ViewType::Dyn(t.clone()),
        })
    }

    /// 遍历全部节点，动态视图将会被立即执行。
    pub fn visit(&self, mut f: impl FnMut(&N)) {
        self.visit_impl(&mut f);
    }

    /// 使用 `&mut impl ...` 防止递归调用时 `f` 的类型无限嵌套。
    fn visit_impl(&self, f: &mut impl FnMut(&N)) {
        match &self.0 {
            ViewType::Node(t) => f(t),
            ViewType::Fragment(t) => t.iter().for_each(|t| t.visit_impl(f)),
            ViewType::Dyn(t) => t().visit_impl(f),
        }
    }

    /// 将全部节点并逐个附加至 `parent`，动态试图会被立即执行。
    pub fn append_to(&self, parent: &N) {
        self.visit(|t| parent.append_child(t));
    }

    pub fn first(&self) -> N {
        let mut current = self.clone();
        loop {
            match current.0 {
                ViewType::Node(t) => return t,
                ViewType::Fragment(t) => current = t.first().unwrap().clone(),
                ViewType::Dyn(t) => current = t(),
            }
        }
    }

    pub fn last(&self) -> N {
        let mut current = self.clone();
        loop {
            match current.0 {
                ViewType::Node(t) => return t,
                ViewType::Fragment(t) => current = t.last().unwrap().clone(),
                ViewType::Dyn(t) => current = t(),
            }
        }
    }
}
