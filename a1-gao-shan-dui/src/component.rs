use crate::{
    template::{GlobalTemplates, RenderOutput, Template, TemplateContent},
    untrack, GenericNode, NodeType, View,
};

/// 构建视图的核心机制，每个组件可以被构建成一个 [`Template`]。
pub trait GenericComponent<N: GenericNode>: 'static + Sized {
    /// 构建模板，每次构建都应该返回相同的结果。
    fn build_template(self) -> Template<N>;

    /// 将当前组件及其 ID 打包成 [`DynComponent`]。
    fn into_dyn_component(self) -> DynComponent<N> {
        DynComponent {
            template: self.build_template(),
        }
    }

    /// 初始化模板并渲染。
    fn render(self) -> View<N> {
        self.into_dyn_component().render()
    }

    /// 初始化模板并将渲染的视图附加到 `parent`。
    fn render_to(self, parent: &N) {
        untrack(|| self.into_dyn_component().render_to(parent));
    }
}

/// 将某个组件打包成 [`DynComponent`]。
pub struct DynComponent<N> {
    template: Template<N>,
}

impl<N: GenericNode> GenericComponent<N> for DynComponent<N> {
    fn build_template(self) -> Template<N> {
        self.template
    }

    fn into_dyn_component(self) -> DynComponent<N> {
        self
    }

    fn render(self) -> View<N> {
        let Self {
            template: Template { id, init, render },
        } = self;
        // 1) 初始化阶段
        let TemplateContent { container } = {
            let init_template = move || {
                let container = N::create(NodeType::Template(id.map(|id| id.data()).unwrap_or("")));
                init().append_to(&container);
                TemplateContent { container }
            };
            if let Some(id) = id {
                // 拷贝或者初始化模板。
                GlobalTemplates::clone_or_insert_with(id, init_template)
            } else {
                // 直接初始化模板而不储存。
                init_template()
            }
        };
        // 2) 渲染阶段
        let RenderOutput { view, next } = render(container.first_child());
        // next 应该指向最后一个子结点的下一个节点，即 None
        debug_assert!(next.is_none());
        view
    }

    fn render_to(self, parent: &N) {
        self.render().append_to(parent);
    }
}
