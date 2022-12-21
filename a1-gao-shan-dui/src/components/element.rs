use crate::{
    template::{RenderOutput, Template},
    GenericComponent, GenericElement, GenericNode, Scope, View,
};

type InitAndRenderRoot<N> = (Box<dyn FnOnce() -> N>, Box<dyn FnOnce(N) -> View<N>>);

/// 将某个元素包装成成一个组件。
pub struct Element<N> {
    cx: Scope,
    /// 初始化和渲染渲染这个元素。
    init_and_render_root: Option<InitAndRenderRoot<N>>,
    /// 初始化子节点，即将它们附加到当前元素。
    init_children: Box<dyn FnOnce(&N)>,
    /// 渲染子结点，接受第一个子结点作为参数，返回最后一个子结点的下一个节点，即 None。
    render_children: Box<dyn FnOnce(Option<N>) -> Option<N>>,
}

impl<N: GenericNode> GenericComponent<N> for Element<N> {
    fn build_template(self) -> Template<N> {
        let Self {
            init_and_render_root,
            init_children,
            render_children,
            ..
        } = self;
        let (init, render) = init_and_render_root.expect("`Element` 没有指定 `root`");
        Template {
            init: Box::new(|| {
                let root = init();
                init_children(&root);
                View::node(root)
            }),
            render: Box::new(|root| {
                let first_child = root.first_child();
                let next = root.next_sibling();
                let view = render(root);
                let last_child = render_children(first_child);
                debug_assert!(last_child.is_none());
                RenderOutput { next, view }
            }),
        }
    }
}

/// 创建一个新的 [`struct@Element`]。
///
/// 必须提供的属性：
///
/// - [`Element::root`] / [`Element::root_with`]
#[allow(non_snake_case)]
pub fn Element<N: GenericNode>(cx: Scope) -> Element<N> {
    Element {
        cx,
        init_and_render_root: None,
        init_children: Box::new(|_| {}),
        // 跳过第一次 render
        render_children: Box::new(|first| first),
    }
}

impl<N: GenericNode> Element<N> {
    pub fn build(self) -> Self {
        self
    }

    /// 设定此组件的根节点。
    pub fn root<E: GenericElement<N>>(self, render: impl 'static + FnOnce(E) -> E) -> Self {
        self.root_with::<E>(move |el| View::node(render(el).into_node()))
    }

    /// 使用自定的函数渲染此组件的根节点。
    pub fn root_with<E: GenericElement<N>>(
        mut self,
        render: impl 'static + FnOnce(E) -> View<N>,
    ) -> Self {
        if self.init_and_render_root.is_some() {
            panic!("`Element` 有且只能有一个 `root`");
        }
        self.init_and_render_root = Some((
            Box::new(move || E::create(self.cx).into_node()),
            Box::new(move |node| render(E::create_with_node(self.cx, node))),
        ));
        self
    }

    /// 添加一个子结点。
    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Self {
        let Template { init, render, .. } = child.build_template();
        self.init_children = Box::new(move |root| {
            (self.init_children)(root);
            init().append_to(root);
        });
        self.render_children = Box::new(move |first| {
            let node = (self.render_children)(first);
            render(node.unwrap()).next
        });
        self
    }
}
