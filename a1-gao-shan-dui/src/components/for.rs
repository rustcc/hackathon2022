use crate::{
    untrack, utils::ViewParentExt, view, DynComponent, GenericComponent, GenericElement,
    GenericNode, IntoReactive, Reactive, Scope, View,
};
use std::{collections::HashMap, hash::Hash};

define_placeholder!(Placeholder("空 `akun::For` 的占位符"));

/// 动态更新的列表。
pub struct For<N: GenericNode, T: 'static, K: 'static> {
    cx: Scope,
    each: Option<Reactive<Vec<T>>>,
    key: Option<Box<dyn Fn(&T) -> K>>,
    children: Option<Box<dyn Fn(&T) -> DynComponent<N>>>,
}

/// 创建一个 [`struct@For`] 组件。
///
/// 该组件根据 [`each`] 提供的元素及其键值动态更新呈现的视图。
///
/// 必须提供的属性：
/// - [`each`] : 输入的数组
/// - [`key`] : 取得元素的键
/// - [`child`] : 从元素产生一个 [`View`]
///
/// [`each`]: For::each
/// [`key`]: For::key
/// [`child`]: For::child
#[allow(non_snake_case)]
pub fn For<N, K, V>(cx: Scope) -> For<N, K, V>
where
    N: GenericNode,
{
    For {
        cx,
        each: None,
        key: None,
        children: None,
    }
}

impl<N, T, K> For<N, T, K>
where
    N: GenericNode,
    T: Clone,
    K: Clone + Eq + Hash,
{
    pub fn build(self) -> impl GenericComponent<N> {
        let Self {
            cx,
            each,
            key,
            children,
        } = self;
        let each = each.expect("`For` 没有指定 `each`");
        let fn_key = key.expect("`For` 没有指定 `key`");
        let fn_view = children.expect("`For` 没有指定 `child`");
        view(cx).root_with(move |placeholder: Placeholder<N>| {
            let placeholder = View::node(placeholder.into_node());
            let dyn_view = View::dyn_(cx, placeholder.clone());
            cx.create_effect({
                let mounted_view = dyn_view.clone();
                let mut mounted_fragment = Vec::new();
                let mut cached_views = HashMap::<K, Cached<N>>::new();
                move || {
                    let each = each.clone().into_value();
                    untrack(|| {
                        let current_view = mounted_view.get();
                        let parent = current_view.parent();
                        // 复用或创建视图
                        let mut new_fragment = Vec::with_capacity(each.len());
                        for val in each.iter() {
                            let k = fn_key(val);
                            let Cached { view, moved } =
                                cached_views.entry(k.clone()).or_insert_with(|| Cached {
                                    view: fn_view(val).render(),
                                    moved: false,
                                });
                            if *moved {
                                // 忽略重复的键值
                                continue;
                            }
                            *moved = true;
                            new_fragment.push(Pair { k, v: view.clone() });
                        }
                        let new_view;
                        if new_fragment.is_empty() {
                            if mounted_fragment.is_empty() {
                                return;
                            }
                            // 用占位符替换空视图
                            parent.replace_child(&placeholder, &current_view);
                            mounted_fragment.clear();
                            cached_views.clear();
                            new_view = placeholder.clone();
                        } else {
                            new_view = View::fragment(
                                new_fragment.iter().map(|pair| pair.v.clone()).collect(),
                            );
                            if mounted_fragment.is_empty() {
                                // 新视图替换掉占位符
                                parent.replace_child(&new_view, &placeholder);
                            } else {
                                // 新旧视图作 diff
                                reconcile(
                                    &mut cached_views,
                                    parent.as_ref(),
                                    &mounted_fragment,
                                    &new_fragment,
                                );
                            }
                            mounted_fragment = new_fragment;
                        }
                        // 重置缓存状态
                        for v in cached_views.values_mut() {
                            debug_assert!(v.moved);
                            v.moved = false;
                        }
                        // 更新之后缓存的视图应该与挂载的片段一致
                        debug_assert_eq!(cached_views.len(), mounted_fragment.len());
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
            panic!("`For` 有且只能有一个 `each`");
        }
        self.each = Some(each.into_reactive());
        self
    }

    pub fn key(mut self, key: impl 'static + Fn(&T) -> K) -> Self {
        if self.key.is_some() {
            panic!("`For` 有且只能有一个 `key`");
        }
        self.key = Some(Box::new(key));
        self
    }

    pub fn child<C: GenericComponent<N>>(mut self, child: impl 'static + Fn(&T) -> C) -> Self {
        if self.children.is_some() {
            panic!("`For` 有且只能有一个 `child`");
        }
        self.children = Some(Box::new(move |val| child(val).into_dyn_component()));
        self
    }
}

#[derive(Clone)]
struct Cached<N: GenericNode> {
    view: View<N>,
    moved: bool,
}

#[derive(Clone)]
struct Pair<K, V> {
    k: K,
    v: V,
}

/// 对新旧视图作 `diff`，移动、删除或者插入新的视图。
fn reconcile<N, K>(
    cached_views: &mut HashMap<K, Cached<N>>,
    parent: Option<&N>,
    a: &Vec<Pair<K, View<N>>>,
    b: &Vec<Pair<K, View<N>>>,
) where
    N: GenericNode,
    K: Clone + Eq + Hash,
{
    let mut a_start = 0;
    let mut a_end = a.len();
    let mut b_start = 0;
    let mut b_end = b.len();
    let mut b_map = None::<HashMap<K, usize>>;

    while a_start < a_end || b_start < b_end {
        if a_start == a_end {
            // 插入新增的视图
            let first = a.get(a_start).or_else(|| a.last()).unwrap().v.first();
            for Pair { v, .. } in b[b_start..b_end].iter() {
                parent.insert_before(v, Some(&first));
                b_start += 1;
            }
            break;
        } else if b_start == b_end {
            // 移除多余的视图
            for Pair { k, v } in a[a_start..a_end].iter() {
                if b_map.as_ref().map(|m| m.contains_key(k)) != Some(true) {
                    parent.remove_child(v);
                    cached_views.remove(k);
                }
                a_start += 1;
            }
            break;
        }

        let a_start_kv = &a[a_start];
        let a_end_kv = &a[a_end - 1];
        let b_start_kv = &b[b_start];
        let b_end_kv = &b[b_end - 1];
        if a_start_kv.k == b_start_kv.k {
            // 跳过相同的前缀
            a_start += 1;
            b_start += 1;
        } else if a_end_kv.k == b_end_kv.k {
            // 跳过相同的后缀
            a_end -= 1;
            b_end -= 1;
        } else if a_start_kv.k == b_end_kv.k && a_end_kv.k == b_start_kv.k {
            // 首尾交换
            let start_next = a_start_kv.v.next_sibling();
            let end_next = a_end_kv.v.next_sibling();
            parent.insert_before(&a_start_kv.v, end_next.as_ref());
            parent.insert_before(&a_end_kv.v, start_next.as_ref());
            a_start += 1;
            b_start += 1;
            a_end -= 1;
            b_end -= 1;
        } else {
            let map = &*b_map.get_or_insert_with(|| {
                b[b_start..b_end]
                    .iter()
                    .enumerate()
                    .map(|(i, p)| (p.k.clone(), b_start + i))
                    .collect()
            });
            if let Some(&index) = map.get(&a_start_kv.k) {
                if index > b_start && index < b_end {
                    // 插入中间新增的视图
                    let first = a_start_kv.v.first();
                    for Pair { v, .. } in b[b_start..index].iter() {
                        parent.insert_before(v, Some(&first));
                        b_start += 1;
                    }
                } else {
                    // 忽略已经插入的视图
                    a_start += 1;
                }
            } else {
                // 移除多余视图
                parent.remove_child(&a_start_kv.v);
                cached_views.remove(&a_start_kv.k);
                a_start += 1;
            }
        }
    }

    // 做一些检查
    debug_assert_eq!(a_start, a_end);
    debug_assert_eq!(b_start, b_end);
}
