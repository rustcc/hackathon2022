use crate::{
    untrack, utils::ViewParentExt, view, DynComponent, GenericComponent, GenericElement,
    GenericNode, IntoReactive, Reactive, Scope, View,
};

define_placeholder!(Placeholder("空 `akun::List` 的占位符"));

/// 动态更新的列表。
pub struct List<N: GenericNode, T: 'static> {
    cx: Scope,
    each: Option<Reactive<Vec<T>>>,
    children: Option<Box<dyn Fn(&T, usize) -> DynComponent<N>>>,
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
            let dyn_view = View::dyn_(cx, placeholder.clone());
            cx.create_effect({
                let mounted_view = dyn_view.clone();
                let mut mounted_fragment = Vec::new();
                move || {
                    // 只需要跟踪 `each` 的变化。
                    let each = each.clone().into_value();
                    untrack(|| {
                        let current_view = mounted_view.get();
                        let parent = current_view.parent();
                        let mounted_len = mounted_fragment.len();
                        let mut new_len = 0;
                        let next_sibling = current_view.next_sibling();
                        for val in each.iter() {
                            // 将新增的视图挂载到当前视图之后。
                            if new_len >= mounted_len {
                                let new_view = fn_view(val, new_len).render();
                                parent.insert_before(&new_view, next_sibling.as_ref());
                                mounted_fragment.push(new_view);
                            }
                            new_len += 1;
                        }
                        let new_view;
                        if new_len == mounted_len {
                            return;
                        } else if new_len == 0 {
                            if mounted_len == 0 {
                                return;
                            }
                            // 用占位符替换掉空的视图。
                            parent.replace_child(&placeholder, &current_view);
                            mounted_fragment.clear();
                            new_view = placeholder.clone();
                        } else {
                            if new_len < mounted_len {
                                // 移除多余的视图。
                                for view in mounted_fragment.drain(new_len..) {
                                    parent.remove_child(&view);
                                }
                            } else if mounted_len == 0 {
                                // new_len > mounted_len，移除占位符。
                                parent.remove_child(&placeholder);
                            }
                            new_view = View::fragment(mounted_fragment.clone());
                        }
                        debug_assert!(new_view.check_mount_order());
                        mounted_view.set(new_view);
                    });
                }
            });
            dyn_view.into()
        })
    }

    pub fn each<E: IntoReactive<Vec<T>>>(mut self, each: E) -> Self {
        if self.each.is_some() {
            panic!("`List` 有且只能有一个 `each`");
        }
        self.each = Some(each.into_reactive());
        self
    }

    pub fn child<C: GenericComponent<N>>(
        mut self,
        child: impl 'static + Fn(&T, usize) -> C,
    ) -> Self {
        if self.children.is_some() {
            panic!("`List` 有且只能有一个 `child`");
        }
        self.children = Some(Box::new(move |t, i| child(t, i).into_dyn_component()));
        self
    }
}
