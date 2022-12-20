use crate::{
    untrack, utils::ViewParentExt, view, GenericComponent, GenericElement, GenericNode,
    IntoReactive, Reactive, Scope, View,
};

define_placeholder!(Placeholder("空 `akun::List` 的占位符"));

type ReactiveList<T> = Reactive<Vec<T>>;

/// 动态更新的列表。
pub struct List<N, T: 'static> {
    cx: Scope,
    each: Option<ReactiveList<T>>,
    children: Option<Box<dyn Fn(&T, usize) -> View<N>>>,
}

/// 创建一个 [`struct@List`] 组件。
///
/// 该组件根据 [`each`] 长度的变化动态的增加/删除挂载的视图。
///
/// 必须提供的属性：
/// - [`each`] : 输入的数组
/// - [`child`] : 根据数组中的元素产生一个 [`View`]。
///
/// [`each`]: List::each
/// [`child`]: List::child
#[allow(non_snake_case)]
pub fn List<N, T>(cx: Scope) -> List<N, T>
where
    N: GenericNode,
{
    List {
        cx,
        each: None,
        children: None,
    }
}

impl<N, T> List<N, T>
where
    N: GenericNode,
    T: Clone,
{
    pub fn build(self) -> impl GenericComponent<N> {
        let Self { cx, each, children } = self;
        let each = each.expect("`List` 没有指定 `each`");
        let fn_view = children.expect("`List` 没有指定 `child`");
        view(cx).root_with(move |placeholder: Placeholder<N>| {
            let placeholder = View::node(placeholder.into_node());
            let mut mounted_fragment = Vec::new();
            let mounted_view = cx.create_signal(placeholder.clone());
            cx.create_effect(move || {
                // 只需要跟踪 `each` 的变化。
                let each = each.clone().into_value();
                untrack(|| {
                    let current_view = mounted_view.get();
                    let parent = current_view.parent();
                    let next_sibling = current_view.next_sibling();
                    let mounted_len = mounted_fragment.len();
                    let mut new_len = 0;
                    for val in each.iter() {
                        // 将新增的视图挂载到当前视图之后。
                        if new_len >= mounted_len {
                            let new_view = fn_view(val, new_len);
                            parent.insert_before(&new_view, next_sibling.as_ref());
                            mounted_fragment.push(new_view);
                        }
                        new_len += 1;
                    }
                    if new_len == 0 {
                        // 用占位符替换掉空的视图。
                        if mounted_len != 0 {
                            parent.replace_child(&placeholder, &current_view);
                            mounted_fragment.clear();
                            mounted_view.set(placeholder.clone());
                        }
                    } else if new_len > mounted_len {
                        // 移除占位符。
                        if mounted_len == 0 {
                            parent.remove_child(&placeholder);
                        }
                        mounted_view.set(View::fragment(mounted_fragment.clone()))
                    } else if new_len < mounted_len {
                        // 移除多余的视图。
                        for view in mounted_fragment.drain(new_len..) {
                            parent.remove_child(&view);
                        }
                        mounted_view.set(View::fragment(mounted_fragment.clone()))
                    }
                });
            });
            View::from(mounted_view)
        })
    }

    pub fn each<E: IntoReactive<Vec<T>>>(mut self, each: E) -> Self {
        if self.each.is_some() {
            panic!("`List` 有且只能有一个 `each`");
        }
        self.each = Some(each.into_reactive());
        self
    }

    pub fn child<C: GenericComponent<N>>(mut self, child: impl 'static + Fn(&T, usize) -> C) -> Self {
        if self.children.is_some() {
            panic!("`List` 有且只能有一个 `child`");
        }
        self.children = Some(Box::new(move |t, i| child(t, i).render()));
        self
    }
}
