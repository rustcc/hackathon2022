use crate::{GenericNode, Signal};
use std::rc::Rc;

type ViewDyn<N> = Rc<dyn Fn() -> View<N>>;

/// 细粒度更新视图的核心机制。每个组件通过 [`View`] 与其父组件同步视图，以保证父
/// 组件可将其视为一个整体来执行动态更新。
#[derive(Clone)]
pub struct View<N>(ViewType<N>);

use ViewType as VT;

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
        Self(VT::Dyn(Rc::new(f)))
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
        Self(VT::Node(node))
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
        Self(VT::fragment(views))
    }

    /// 对所有的节点执行 [`deep_clone`]，动态视图则拷贝其引用。
    ///
    /// [`deep_clone`]: [`GenericNode::deep_clone`]
    pub fn deep_clone(&self) -> View<N> {
        Self(match &self.0 {
            VT::Node(t) => VT::Node(t.deep_clone()),
            VT::Fragment(t) => VT::fragment(t.iter().map(|t| t.deep_clone()).collect::<Vec<_>>()),
            VT::Dyn(t) => VT::Dyn(t.clone()),
        })
    }

    /// 遍历全部节点，动态视图将会被立即执行。
    pub fn visit(&self, mut f: impl FnMut(&N)) {
        self.visit_impl(&mut f);
    }

    /// 使用 `&mut impl ...` 防止递归调用时 `f` 的类型无限嵌套。
    fn visit_impl(&self, f: &mut impl FnMut(&N)) {
        match &self.0 {
            VT::Node(t) => f(t),
            VT::Fragment(t) => t.iter().for_each(|t| t.visit_impl(f)),
            VT::Dyn(t) => t().visit_impl(f),
        }
    }

    /// 检查两个 [`View`] 的引用是否相等。
    pub fn ref_eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (VT::Node(t1), VT::Node(t2)) => t1.eq(t2),
            (VT::Fragment(t1), VT::Fragment(t2)) => Rc::ptr_eq(t1, t2),
            // NOTE: https://rust-lang.github.io/rust-clippy/master/index.html#vtable_address_comparisons
            (VT::Dyn(t1), VT::Dyn(t2)) => {
                let ptr1 = Rc::as_ptr(t1).cast::<()>();
                let ptr2 = Rc::as_ptr(t2).cast::<()>();
                ptr1 == ptr2
            }
            _ => false,
        }
    }

    pub fn first(&self) -> N {
        let mut current = self.clone();
        loop {
            match current.0 {
                VT::Node(t) => return t,
                VT::Fragment(t) => current = t.first().unwrap().clone(),
                VT::Dyn(t) => current = t(),
            }
        }
    }

    pub fn last(&self) -> N {
        let mut current = self.clone();
        loop {
            match current.0 {
                VT::Node(t) => return t,
                VT::Fragment(t) => current = t.last().unwrap().clone(),
                VT::Dyn(t) => current = t(),
            }
        }
    }

    pub fn parent(&self) -> Option<N> {
        self.first().parent()
    }

    pub fn next_sibling(&self) -> Option<N> {
        self.last().next_sibling()
    }

    /// 将全部节点并逐个附加至 `parent`，动态试图会被立即执行。
    pub fn append_to(&self, parent: &N) {
        self.visit(|t| parent.append_child(t));
    }

    pub fn replace_with(&self, parent: &N, new_view: &Self) {
        if let (VT::Node(old), VT::Node(new)) = (&self.0, &new_view.0) {
            parent.replace_child(new, old);
        } else {
            new_view.move_before(parent, Some(&self.first()));
            self.remove_from(parent);
        }
    }

    pub fn remove_from(&self, parent: &N) {
        self.visit(|t| parent.remove_child(t));
    }

    pub fn move_before(&self, parent: &N, position: Option<&N>) {
        self.visit(|t| parent.insert_before(t, position));
    }
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
