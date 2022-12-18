use crate::{
    elements::IntoReactive,
    template::{Template, TemplateId},
    DynComponent, Element, GenericComponent, GenericElement, GenericNode, Scope,
};

pub fn view_element<N, E>(
    cx: Scope,
    _marker: fn(Scope) -> E,
    render: impl 'static + FnOnce(E) -> E,
) -> Element<N>
where
    N: GenericNode,
    E: GenericElement<N>,
{
    Element(cx).root(render)
}

pub fn view_text<N, V: IntoReactive<String>>(cx: Scope, data: V) -> Element<N>
where
    N: GenericNode,
{
    let reactive = data.into_reactive();
    Element(cx).root(|text: crate::elements::text<N>| text.data(reactive))
}

pub struct ViewRoot<N> {
    id: Option<TemplateId>,
    children: Option<DynComponent<N>>,
}

#[allow(non_snake_case)]
pub fn ViewRoot<N: GenericNode>(_: Scope) -> ViewRoot<N> {
    ViewRoot {
        id: None,
        children: None,
    }
}

impl<N: GenericNode> GenericComponent<N> for ViewRoot<N> {
    fn build_template(self) -> Template<N> {
        if self.id.is_none() {
            panic!("未指定 `view!` 的 ID");
        }
        let template = self
            .children
            .expect("未指定 `view!` 的根组件")
            .build_template();
        Template {
            id: self.id,
            ..template
        }
    }
}

impl<N: GenericNode> ViewRoot<N> {
    pub fn build(self) -> Self {
        self
    }

    pub fn id(mut self, id: TemplateId) -> Self {
        if self.id.is_some() {
            panic!("`view!` 有且只能有一个 ID")
        }
        self.id = Some(id);
        self
    }

    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Self {
        if self.children.is_some() {
            panic!("`view!` 有且只能有一个组件");
        }
        self.children = Some(child.into_dyn_component());
        self
    }
}
