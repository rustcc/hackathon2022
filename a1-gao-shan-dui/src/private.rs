use crate::{
    elements::IntoReactive,
    template::{Template, TemplateId},
    DynComponent, Element, GenericComponent, GenericElement, GenericNode, Property, Scope,
};

pub fn view_element<N, E>(
    cx: Scope,
    create: fn(Scope) -> E,
    props: impl 'static + FnOnce(E) -> E,
    children: impl 'static + FnOnce(Element<N>) -> Element<N>,
) -> Element<N>
where
    N: GenericNode,
    E: GenericElement<N>,
{
    let _ = create;
    let element = Element(cx).root(props);
    children(element)
}

pub fn view_component<Init, U1, U2, Final>(
    cx: Scope,
    create: fn(Scope) -> Init,
    props: impl 'static + FnOnce(Init) -> U1,
    children: impl 'static + FnOnce(U1) -> U2,
    build: impl 'static + FnOnce(U2) -> Final,
) -> Final {
    let component = create(cx);
    let u1 = props(component);
    let u2 = children(u1);
    build(u2)
}

pub fn view_text<N, V: IntoReactive<Property>>(cx: Scope, data: V) -> Element<N>
where
    N: GenericNode,
{
    let reactive = data.into_reactive();
    Element(cx).root(|text: crate::elements::text<N>| text.data(reactive))
}

pub struct ViewRoot<N: GenericNode> {
    id: Option<fn() -> TemplateId>,
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
    fn id(&self) -> Option<TemplateId> {
        Some(self.id.expect("`view!` 没有指定 ID")())
    }

    fn build_template(self) -> Template<N> {
        self.children
            .expect("`view!` 没有指定根组件")
            .build_template()
    }
}

impl<N: GenericNode> ViewRoot<N> {
    pub fn build(self) -> Self {
        self
    }

    pub fn id(mut self, id: fn() -> TemplateId) -> Self {
        if self.id.is_some() {
            panic!("`view!` 有且只能有一个 ID")
        }
        self.id = Some(id);
        self
    }

    pub fn child<C: GenericComponent<N>>(mut self, child: C) -> Self {
        if self.children.is_some() {
            panic!("`view!` 有且只能有一个根组件");
        }
        self.children = Some(child.into_dyn_component());
        self
    }
}
