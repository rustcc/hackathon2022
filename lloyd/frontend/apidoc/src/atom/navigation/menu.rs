use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props<'a> {
    pub items: Vec<Item>,
    pub selected: &'a Signal<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
    pub id: Option<String>,

    pub name: String,

    pub active: bool,

    pub childs: Vec<Item>,

    pub amount: i32,
}

#[component(inline_props)]
fn Item<'a, G: Html>(
    cx: Scope<'a>,
    item: &'a Signal<Item>,
    selected: &'a Signal<String>,
) -> View<G> {
    let active = create_signal(cx, item.get().active);
    let ul_class = create_memo(cx, || {
        if !*active.get() || item.get().childs.is_empty() {
            return String::from("is-hidden");
        }
        String::from("menu-list")
    });
    let a_class = create_memo(cx, || {
        if let Some(item_id) = &item.get().id {
            if !selected.get().as_str().is_empty() && item_id.eq(selected.get().as_str()) {
                return String::from("is-active menu-list");
            }
        }
        String::default()
    });
    let down_span_class = create_memo(cx, || {
        if item.get().childs.is_empty() {
            return String::from("is-hidden");
        }
        if *active.get() {
            return String::from("icon");
        }
        String::from("is-hidden")
    });
    let right_span_class = create_memo(cx, || {
        if item.get().childs.is_empty() {
            return String::from("is-hidden");
        }
        if *active.get() {
            return String::from("is-hidden");
        }
        String::from("icon is-align-content-end")
    });

    let amount = create_memo(cx, || {
        if item.get().childs.is_empty() {
            return String::from("is-hidden");
        }
        String::from("tag is-success is-light")
    });

    view! {cx,
        li {
            a (class = a_class.get(), on:click = move |_| {
                active.set(!*active.get());
                let item_id = &item.get().id;
                if let Some(item_id) = item_id {
                    if !item_id.eq(selected.get().as_str()) {
                        selected.set(item_id.to_string());
                    }
                }
            }) {(item.get().name)
                span (class = amount.get(), style = "margin-left: 3px; margin-right: 3px;") {
                    (item.get().amount)
                }

                span (class = right_span_class.get()) {
                    svg (class="svg-inline--fa fa-angle-right fa-w-8", aria-hidden="true", data-prefix="fas", data-icon="angle-right", role="img", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 256 512", data-fa-i2svg="") {
                        path (fill="currentColor", d="M224.3 273l-136 136c-9.4 9.4-24.6 9.4-33.9 0l-22.6-22.6c-9.4-9.4-9.4-24.6 0-33.9l96.4-96.4-96.4-96.4c-9.4-9.4-9.4-24.6 0-33.9L54.3 103c9.4-9.4 24.6-9.4 33.9 0l136 136c9.5 9.4 9.5 24.6.1 34z") {}
                    }
                }
                span (class = down_span_class.get()) {
                    svg (class="svg-inline--fa fa-angle-down fa-w-10", aria-hidden="true", data-prefix="fas", data-icon="angle-down", role="img", xmlns="http://www.w3.org/2000/svg", viewBox="0 0 320 512", data-fa-i2svg="") {
                        path (fill="currentColor", d="M143 352.3L7 216.3c-9.4-9.4-9.4-24.6 0-33.9l22.6-22.6c9.4-9.4 24.6-9.4 33.9 0l96.4 96.4 96.4-96.4c9.4-9.4 24.6-9.4 33.9 0l22.6 22.6c9.4 9.4 9.4 24.6 0 33.9l-136 136c-9.2 9.4-24.4 9.4-33.8 0z") {}
                    }
                }
            }
            ul (class = ul_class.get()) {
                List(items = item.get().childs.clone(), selected = selected)
            }
        }
    }
}

#[component]
fn List<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    let data = props
        .items
        .iter()
        .map(|i| create_signal(cx, i.clone()))
        .collect::<Vec<_>>();
    let data = create_signal(cx, data);
    view! {cx,
        Indexed(
            iterable=data,
            view= move |cx, item| view! { cx,
                Item(item = item, selected = props.selected)
            },
        )
    }
}

#[component]
pub fn NavMenu<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! {cx,
        aside (class="menu", style="user-select:none;") {
            ul (class = "menu-list") {
                List(items = props.items, selected = props.selected)
            }
        }
    }
}
