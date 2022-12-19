use crate::{
    untrack, view, DynComponent, GenericComponent, GenericElement, GenericNode, IntoReactive,
    Reactive, Scope, Value, View,
};

define_placeholder!(Placeholder("空 `akun::Show` 组件的占位符"));

/// 创建一个 [`struct@Show`] 组件。
///
/// 动态检查每个分支的条件被渲染特定的组件，仅接受 [`If`] 或 [`Else`] 作为 [`child`]。
///
/// [`If`]: If()
/// [`Else`]: Else()
/// [`child`]: Show::child
#[allow(non_snake_case)]
pub fn Show<N: GenericNode>(cx: Scope) -> Show<N> {
    Show {
        cx,
        children: Default::default(),
    }
}

/// 动态检查每个分支的条件被渲染特定的组件。
pub struct Show<N> {
    cx: Scope,
    children: Vec<ShowChild<N>>,
}

pub struct ShowChild<N> {
    cond: Reactive<bool>,
    content: DynComponent<N>,
}

struct Branch<N> {
    cond: Reactive<bool>,
    view: View<N>,
}

impl<N> Show<N>
where
    N: GenericNode,
{
    pub fn build(self) -> impl GenericComponent<N> {
        let Self { cx, children } = self;
        // 使用占位符来标定需要挂载的位置
        view(cx).root_with(move |placeholder: Placeholder<N>| {
            let placeholder = View::node(placeholder.into_node());
            let branches = children
                .into_iter()
                .map(|ShowChild { cond, content }| Branch {
                    cond,
                    // 忽略渲染时产生的副作用，通常是读取动态视图时产生的
                    view: untrack(|| content.render()),
                })
                // 如果 `children` 为空则用占位符替代
                .chain(Some(Branch {
                    cond: Value(true),
                    view: placeholder.clone(),
                }))
                .collect::<Vec<_>>();
            let mounted_view = cx.create_signal(placeholder);
            // `current_view` 被卸载之后，如果重新渲染被触发，则在一个模板中执行渲染。
            let mut unmounted_parent = None;
            cx.create_effect(move || {
                for branch in branches.iter() {
                    let Branch::<N> {
                        cond,
                        view: new_view,
                    } = branch;
                    if cond.clone().into_value() {
                        // 我们只需要在 `Show` 条件更新时获取动态视图最新的内容，而不需要
                        // 一直跟踪该视图的变化。
                        untrack(|| {
                            let current_view = mounted_view.get();
                            // 需要注意的是，`current_view` 的 `parent` 并不一定为
                            // `placeholder` 最初的 `parent`，因为一个组件可能在模板
                            // 节点中完成第一次渲染，然后整体被挂载到其他的位置。故每次更新
                            // 视图时都需要重新获取 `current_view` 的父级。
                            let parent = current_view.parent_or(|| {
                                unmounted_parent
                                    .get_or_insert_with(N::empty_template)
                                    .clone()
                            });
                            if !current_view.ref_eq(new_view) {
                                current_view.replace_with(&parent, new_view);
                                mounted_view.set(new_view.clone());
                            }
                        });
                        break;
                    }
                }
            });
            View::from(mounted_view)
        })
    }
}

impl<N> Show<N> {
    pub fn child(mut self, child: ShowChild<N>) -> Show<N> {
        self.children.push(child);
        self
    }
}

/// 创建一个新的 [`struct@If`] 组件。
///
/// 如果 [`Show`] 中此组件以前的所有分支均不会被挂载，且此分支的条件为 `true`，那么此分支将会被挂载。
///
/// 必须提供的属性：
/// - [`when`] : 标定是否需要挂载 [`child`] 的条件
/// - [`child`] : 需要被挂载的组件
///
/// [`Show`]: Show()
/// [`when`]: If::when
/// [`child`]: If::child
#[allow(non_snake_case)]
pub fn If<N: GenericNode>(_: Scope) -> If<N> {
    If {
        when: None,
        children: None,
    }
}

/// 作为 [`Show`] 的条件挂载分支。
///
/// [`Show`]: Show()
pub struct If<N> {
    when: Option<Reactive<bool>>,
    children: Option<DynComponent<N>>,
}

impl<N: GenericNode> If<N> {
    pub fn build(self) -> ShowChild<N> {
        ShowChild {
            cond: self.when.expect("`If` 没有指定 `when`"),
            content: self.children.expect("`If` 没有指定 `child`"),
        }
    }
}

impl<N: GenericNode> If<N> {
    pub fn when<T: IntoReactive<bool>>(mut self, when: T) -> If<N> {
        if self.when.is_some() {
            panic!("`If` 有且只能有一个 `when`");
        }
        self.when = Some(when.into_reactive());
        self
    }

    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> If<N> {
        if self.children.is_some() {
            panic!("`If` 有且只能有一个 `child`");
        }
        self.children = Some(child.into_dyn_component());
        self
    }
}

/// 创建一个新的 [`struct@Else`] 组件。
///
/// 如果 [`Show`] 中此组件以前的所有分支均不会被挂载，那么此分支将会被挂载。
///
/// 必须提供的属性：
/// - [`child`] : 需要被挂载的组件。
///
/// [`Show`]: Show()
/// [`child`]: Else::child
#[allow(non_snake_case)]
pub fn Else<N: GenericNode>(_: Scope) -> Else<N> {
    Else { children: None }
}

/// 作为 [`Show`] 的无条件挂载分支。
///
/// [`Show`]: Show()
pub struct Else<N> {
    children: Option<DynComponent<N>>,
}

impl<N: GenericNode> Else<N> {
    pub fn build(self) -> ShowChild<N> {
        ShowChild {
            cond: Value(true),
            content: self.children.expect("`Else` 没有指定 `child`"),
        }
    }
}

impl<N: GenericNode> Else<N> {
    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Else<N> {
        if self.children.is_some() {
            panic!("`Else` 有且只能有一个 `child`");
        }
        self.children = Some(child.into_dyn_component());
        self
    }
}
