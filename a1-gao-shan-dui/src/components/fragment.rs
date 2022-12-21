use crate::{
    template::{BeforeRendering, RenderOutput, Template},
    GenericComponent, GenericElement, GenericNode, Scope, View,
};

define_placeholder!(Placeholder("空 `akun::Fragment` 组件的占位符"));

type Views<N> = Vec<View<N>>;

/// 将多个组件组合成一个片段。
pub struct Fragment<N: GenericNode> {
    init: Box<dyn FnOnce(&mut Views<N>)>,
    render: Box<dyn FnOnce(BeforeRendering<N>, N, &mut Views<N>) -> Option<N>>,
}

impl<N: GenericNode> GenericComponent<N> for Fragment<N> {
    fn build_template(self) -> Template<N> {
        let Self { init, render } = self;
        Template {
            init: Box::new(move || {
                let mut views = Views::default();
                init(&mut views);
                if views.is_empty() {
                    // 空片段用占位符替代
                    View::node(N::create(Placeholder::<N>::TYPE))
                } else {
                    View::fragment(views)
                }
            }),
            render: Box::new(move |before_rendering, node| {
                let mut views = Views::default();
                let next = render(before_rendering, node, &mut views);
                if views.is_empty() {
                    // 跳过占位符
                    let placeholder = next.unwrap();
                    RenderOutput {
                        next: placeholder.next_sibling(),
                        view: View::node(placeholder),
                    }
                } else {
                    RenderOutput {
                        next,
                        view: View::fragment(views),
                    }
                }
            }),
        }
    }
}

/// 创建一个新的 [`struct@Fragment`]。
#[allow(non_snake_case)]
pub fn Fragment<N: GenericNode>(_: Scope) -> Fragment<N> {
    Fragment {
        init: Box::new(|_| {}),
        // 跳过第一次 render
        render: Box::new(|_, first, _| Some(first)),
    }
}

impl<N: GenericNode> Fragment<N> {
    pub fn build(self) -> Self {
        self
    }

    /// 向此片段中添加一个组件。
    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Self {
        let Template { init, render, .. } = child.build_template();
        self.init = Box::new(move |views| {
            (self.init)(views);
            views.push(init());
        });
        self.render = Box::new(move |before_rendering, first, views| {
            let node = (self.render)(before_rendering, first, views);
            let RenderOutput { next, view } = render(before_rendering, node.unwrap());
            views.push(view);
            next
        });
        self
    }
}
