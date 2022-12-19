use crate::{create_root, template::GlobalTemplates, GenericComponent, Scope};
use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::HtmlTemplateElement;

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap();
    static DOCUMENT: web_sys::Document = WINDOW.with(web_sys::Window::document).unwrap();
    static BODY: web_sys::HtmlElement = DOCUMENT.with(web_sys::Document::body).unwrap();
    static DOM_TEMPLATES: GlobalTemplates<DomNode> = GlobalTemplates::default();
}

pub trait UnwrapThrowValExt<T> {
    fn unwrap_throw_val(self) -> T;
}

impl<T> UnwrapThrowValExt<T> for Result<T, wasm_bindgen::JsValue> {
    fn unwrap_throw_val(self) -> T {
        self.unwrap_or_else(|e| wasm_bindgen::throw_val(e))
    }
}

type StaticStr = &'static str;

/// 处理一个节点引发的事件。
pub struct EventHandler {
    pub handler: Box<dyn FnMut(web_sys::Event)>,
}

/// 呈现视图的核心机制，也是视图中的最小单元。如不特殊指出，一个节点的复制应该是
/// 浅拷贝，即只拷贝其对已挂载节点的引用。
pub trait GenericNode: 'static + Clone + Eq {
    /// 返回该类型对应的全局储存的模板。
    fn global_templates() -> GlobalTemplates<Self>;

    /// 创建一个新的节点。
    fn create(ty: NodeType) -> Self;

    /// 对某节点及其全部子结点执行深度复制。
    fn deep_clone(&self) -> Self;

    /// 设定节点的固有属性，对于 Dom 节点，大部分属性通过此方法进行更新。
    fn set_property(&self, name: &str, val: &str);

    /// 设定节点的任意属性，对于 Dom 节点，`data-*` 属性通过此方法进行更新。
    fn set_attribute(&self, name: &str, val: &str);

    /// 设定节点内部文本。
    fn set_inner_text(&self, data: &str);

    /// 向节点中添加一个 `class`。
    fn add_class(&self, name: &str);

    /// 移除节点中的某个 `class`。
    fn remove_class(&self, name: &str);

    /// 监听节点的某个事件。
    fn listen_event(&self, event: &str, handler: EventHandler);

    /// 返回某个节点的父级。
    fn parent(&self) -> Option<Self>;

    /// 返回某个节点的下一个兄弟节点。
    fn next_sibling(&self) -> Option<Self>;

    /// 返回某个节点的第一个子节点。
    fn first_child(&self) -> Option<Self>;

    /// 向某个节点添加一个子结点。
    fn append_child(&self, child: &Self);

    /// 在节点中移除某个子结点。
    fn remove_child(&self, child: &Self);

    /// 将子结点中 `position` 所在的位置替换为 `node`。
    fn replace_child(&self, node: &Self, position: &Self);

    /// 在子结点中 `position` 所在的位置之前插入 `node`。
    fn insert_before(&self, node: &Self, position: Option<&Self>);
}

/// 节点类型。
pub enum NodeType {
    /// 带有标签的节点，通常是 HTML 元素。
    Tag(StaticStr),
    /// 文本节点。
    Text(StaticStr),
    /// 占位符，通常被动态组件用来确定挂载的位置。
    Placeholder(StaticStr),
    /// 模板节点，对模板节点内容的修改不应该影响实际呈现的视图，且模板应该作为作为
    /// 一个片段被插入到节点树中的任意位置，即插入模板的全部子结点而不是模板节点本
    /// 身。对于 [`DomNode`]，通常是 [`DocumentFragment`]。
    ///
    /// [`DocumentFragment`]: web_sys::DocumentFragment
    Template(StaticStr),
}

/// 使用 [`web_sys::Node`] 实现的 [`GenericNode`]。
#[derive(Clone, Eq, PartialEq)]
pub struct DomNode {
    node: web_sys::Node,
}

impl From<web_sys::Node> for DomNode {
    fn from(node: web_sys::Node) -> Self {
        Self { node }
    }
}

impl GenericNode for DomNode {
    fn global_templates() -> GlobalTemplates<Self> {
        DOM_TEMPLATES.with(Clone::clone)
    }

    fn create(ty: NodeType) -> Self {
        let node: web_sys::Node = DOCUMENT.with(|doc| match ty {
            NodeType::Tag(tag) => doc.create_element(tag).unwrap_throw_val().into(),
            NodeType::Text(data) => doc.create_text_node(data).into(),
            NodeType::Placeholder(data) => doc.create_comment(data).into(),
            NodeType::Template(data) => {
                // Debug 构建时将模板插入 Dom 中便于调试。
                if cfg!(debug_assertions) {
                    let template = doc.create_element("template").unwrap_throw_val();
                    if !data.is_empty() {
                        template
                            .set_attribute("data-akun-template-id", data)
                            .unwrap_throw_val();
                        BODY.with(|body| {
                            body.insert_before(&template, body.first_child().as_ref())
                                .unwrap_throw_val();
                        });
                    }
                    template
                        .unchecked_into::<HtmlTemplateElement>()
                        .content()
                        .into()
                } else {
                    doc.create_document_fragment().into()
                }
            }
        });
        Self { node }
    }

    fn deep_clone(&self) -> Self {
        Self {
            node: self.node.clone_node_with_deep(true).unwrap_throw_val(),
        }
    }

    fn set_property(&self, name: &str, attr: &str) {
        Reflect::set(
            &self.node,
            &JsValue::from_str(name),
            &JsValue::from_str(attr),
        )
        .unwrap_throw_val();
    }

    fn set_attribute(&self, name: &str, val: &str) {
        self.node
            .unchecked_ref::<web_sys::Element>()
            .set_attribute(name, val)
            .unwrap_throw_val();
    }

    fn set_inner_text(&self, data: &str) {
        self.node.set_text_content(Some(data));
    }

    fn add_class(&self, name: &str) {
        self.node
            .unchecked_ref::<web_sys::Element>()
            .class_list()
            .add_1(name)
            .unwrap_throw_val();
    }

    fn remove_class(&self, name: &str) {
        self.node
            .unchecked_ref::<web_sys::Element>()
            .class_list()
            .remove_1(name)
            .unwrap_throw_val();
    }

    fn listen_event(&self, event: &str, handler: EventHandler) {
        self.node
            .add_event_listener_with_callback(
                event,
                &Closure::wrap(handler.handler)
                    .into_js_value()
                    .unchecked_into(),
            )
            .unwrap_throw_val();
    }

    fn parent(&self) -> Option<Self> {
        self.node.parent_node().map(Self::from)
    }

    fn next_sibling(&self) -> Option<Self> {
        self.node.next_sibling().map(Self::from)
    }

    fn first_child(&self) -> Option<Self> {
        self.node.first_child().map(Self::from)
    }

    fn append_child(&self, child: &Self) {
        self.node.append_child(&child.node).unwrap_throw_val();
    }

    fn remove_child(&self, node: &Self) {
        self.node.remove_child(&node.node).unwrap_throw_val();
    }

    fn replace_child(&self, new_node: &Self, old_node: &Self) {
        self.node
            .replace_child(&new_node.node, &old_node.node)
            .unwrap_throw_val();
    }

    fn insert_before(&self, new_node: &Self, ref_node: Option<&Self>) {
        self.node
            .insert_before(&new_node.node, ref_node.map(|node| &node.node))
            .unwrap_throw_val();
    }
}

/// 将组件挂载到 `document.body` 上。
pub fn mount_to_body<C>(f: impl FnOnce(Scope) -> C)
where
    C: GenericComponent<DomNode>,
{
    BODY.with(|body| mount_to(body, f));
}

/// 将组件挂载到 `root` 上。
pub fn mount_to<C>(root: &web_sys::Node, f: impl FnOnce(Scope) -> C)
where
    C: GenericComponent<DomNode>,
{
    let (_, disposer) = create_root(|cx| {
        f(cx).mount_to(&DomNode::from(root.clone()));
    });
    std::mem::forget(disposer);
}
