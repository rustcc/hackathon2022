use crate::{
    template::{RenderOutput, Template},
    GenericComponent, GenericNode, Scope, View,
};

type Views<N> = Vec<View<N>>;

/// 将多个组件组合成一个片段。
pub struct Fragment<N> {
    init: Box<dyn FnOnce(&mut Views<N>)>,
    render: Box<dyn FnOnce(Option<N>, &mut Views<N>) -> Option<N>>,
}

impl<N: GenericNode> GenericComponent<N> for Fragment<N> {
    fn build_template(self) -> Template<N> {
        let Self { init, render } = self;
        Template {
            init: Box::new(|| {
                let mut views = Views::default();
                init(&mut views);
                View::from(views)
            }),
            render: Box::new(|node| {
                let mut views = Views::default();
                let next = render(node, &mut views);
                RenderOutput {
                    next,
                    view: View::from(views),
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
        render: Box::new(|first, _| first),
    }
}

impl<N: GenericNode> Fragment<N> {
    /// 向此片段中添加一个组件。
    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Self {
        let Template { init, render } = child.build_template();
        self.init = Box::new(move |views| {
            (self.init)(views);
            views.push(init());
        });
        self.render = Box::new(move |first, views| {
            let node = (self.render)(first, views);
            let RenderOutput { next, view } = render(node);
            views.push(view);
            next
        });
        self
    }
}
