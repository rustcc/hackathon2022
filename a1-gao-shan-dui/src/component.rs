use crate::{
    template::{
        BeforeRendering, GlobalTemplates, RenderOutput, Template, TemplateContent, TemplateId,
    },
    untrack, GenericNode, NodeType, View,
};

/// 构建视图的核心机制，每个组件可以被构建成一个 [`Template`]。
pub trait GenericComponent<N: GenericNode>: 'static + Sized {
    /// 构建模板，每次构建都应该返回相同的结果。
    fn build_template(self) -> Template<N>;

    fn id(&self) -> Option<TemplateId> {
        None
    }

    /// 将当前组件及其 ID 打包成 [`DynComponent`]。
    fn into_dyn_component(self) -> DynComponent<N> {
        DynComponent {
            id: self.id(),
            template: self.build_template(),
        }
    }

    /// 初始化模板并渲染。
    fn render(self) -> View<N> {
        self.into_dyn_component().render()
    }

    /// 初始化模板，并在渲染阶段执行前将模板挂载到 `parent`。
    fn mount_to(self, parent: &N) -> View<N> {
        self.into_dyn_component().mount_to(parent)
    }
}

/// 将某个组件打包成 [`DynComponent`]。
pub struct DynComponent<N: GenericNode> {
    /// 用于标识该模板的唯一 ID。
    id: Option<TemplateId>,
    template: Template<N>,
}

impl<N: GenericNode> GenericComponent<N> for DynComponent<N> {
    fn id(&self) -> Option<TemplateId> {
        self.id
    }

    fn build_template(self) -> Template<N> {
        self.template
    }

    fn into_dyn_component(self) -> DynComponent<N> {
        self
    }

    fn render(self) -> View<N> {
        self.render_impl(None)
    }

    fn mount_to(self, parent: &N) -> View<N> {
        self.render_impl(Some(parent))
    }
}

impl<N: GenericNode> DynComponent<N> {
    fn render_impl(self, parent: Option<&N>) -> View<N> {
        let Self {
            id,
            template: Template { init, render },
        } = self;
        // 1) 初始化阶段
        let TemplateContent { container } = {
            let init_template = move || {
                let container = N::create(NodeType::Template(id.map(|id| id.data())));
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
        let first_child = container.first_child();
        // 2) 渲染阶段，忽略产生的副作用（通常是读写动态视图）
        let before_rendering = parent
            .map(BeforeRendering::AppendTo)
            .unwrap_or(BeforeRendering::RemoveFrom(&container));
        let RenderOutput { view, next } =
            untrack(|| render(before_rendering, first_child.unwrap()));
        // 子代应该被正确的移除或者转移
        debug_assert!(container.first_child().is_none());
        // next 应该指向最后一个子结点的下一个节点，即 None
        debug_assert!(next.is_none());
        view
    }
}
