use crate::{GenericNode, View};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

thread_local! {
    static GLOBAL_ID: Cell<usize> = Cell::new(0);
}

/// 运行时模板化的核心机制，[`Template`]，由两部分组成：[`Template::init`] 用于初
/// 始化节点树，它返回的 [`View`] 不应该包含任何动态内容；[`Template::render`]
/// 在初始化过的节点树上执行渲染工作，它可以返回任意 [`View`]。
pub struct Template<N> {
    /// 用于标识该模板的唯一 ID。
    pub id: Option<TemplateId>,
    /// 初始化阶段执行，返回需要被模板记录的 [`View`]，且每次调用返回的 [`View`]
    /// 结构保持一致。
    pub init: Box<dyn FnOnce() -> View<N>>,
    /// 渲染阶段执行，接受初始化后的首个节点，返回渲染后的 [`View`] 及其之后的
    /// 第一个兄弟节点。
    pub render: Box<dyn FnOnce(N) -> RenderOutput<N>>,
    // 对于 `init` 和 `render` 我们可以使用静态分发而非动态分发来换取更高的性能，
    // 但在实践中这样会导致闭包类型被层层嵌套，从而严重增加了类型复杂度，甚至一度使
    // 编译器陷入 `core dump` 错误。
}

/// 渲染阶段返回的结果。
pub struct RenderOutput<N> {
    /// 该组件之后的第一个兄弟节点。
    pub next: Option<N>,
    /// 该组件渲染后的视图。
    pub view: View<N>,
}

/// 储存了全部有 [`TemplateId`] 且被初始化的模板。
pub struct GlobalTemplates<N> {
    inner: Rc<RefCell<HashMap<usize, TemplateContent<N>>>>,
}

impl<N: GenericNode> Default for GlobalTemplates<N> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<N> Clone for GlobalTemplates<N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<N: GenericNode> GlobalTemplates<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn clone_or_insert_with(
        id: TemplateId,
        f: impl FnOnce() -> TemplateContent<N>,
    ) -> TemplateContent<N> {
        N::global_templates()
            .inner
            .borrow_mut()
            .entry(id.id)
            .or_insert_with(f)
            .deep_clone()
    }
}

/// 用于标识模板的唯一 ID。
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TemplateId {
    data: &'static str,
    id: usize,
}

impl TemplateId {
    /// 生成一个新的 ID，附加 `data` 用于调试信息。
    pub fn generate(data: &'static str) -> Self {
        Self {
            id: GLOBAL_ID.with(|global_id| {
                let id = global_id.get();
                global_id.set(id + 1);
                id
            }),
            data,
        }
    }

    pub fn data(&self) -> &'static str {
        self.data
    }
}

pub(crate) struct TemplateContent<N> {
    pub container: N,
}

impl<N: GenericNode> TemplateContent<N> {
    pub fn deep_clone(&self) -> Self {
        Self {
            container: self.container.deep_clone(),
        }
    }
}
