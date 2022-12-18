use crate::{
    template::{GlobalTemplates, RenderOutput, Template, TemplateContent, TemplateId},
    untrack, GenericNode, NodeType, View,
};

/// 构建视图的核心机制，每个组件可以被构建成一个 [`Template`]。
pub trait GenericComponent<N: GenericNode>: 'static + Sized {
    fn build_template(self) -> Template<N>;

    /// 用于标识该组件的唯一 ID，每次调用都应该保持一致。
    fn id() -> Option<TemplateId> {
        None
    }

    /// 将当前组件及其 ID 打包成 [`DynComponent`]。
    fn into_dyn_component(self) -> DynComponent<N> {
        DynComponent {
            id: Self::id(),
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

pub struct DynComponent<N> {
    id: Option<TemplateId>,
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
            id,
            template: Template { init, render },
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

impl<N: GenericNode> DynComponent<N> {
    /// 读取当前组件的 ID，调用 [`GenericComponent::id`] 会返回空值。
    pub fn id(&self) -> Option<TemplateId> {
        self.id
    }
}
