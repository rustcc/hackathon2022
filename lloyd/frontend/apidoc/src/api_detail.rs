use std::collections::HashMap;

use common::doc::{ApiFieldType, ApiMember, ApiModel, ApiOperation};
use common::validator::ValidateType;
use derive_more::Display;
use sycamore::prelude::*;
use sycamore::{component, reactive::Scope, view::View, web::Html};

pub const NOT_URL_ID: &str = "//doc";
pub const CLOSE_ID: &str = "//close:";

#[derive(Prop)]
pub struct Props<'a> {
    pub apis: &'a Signal<Vec<ApiOperation>>,
    pub selected: &'a Signal<String>,
    pub doc: ApiNote,
}

#[derive(Debug, Clone)]
pub struct ApiNote {
    pub name: String,
    pub note: String,
}

#[component(inline_props)]
pub fn MemberRow<'a, G: Html>(
    cx: Scope<'a>,
    member: ApiMember,
    serde: bool,
    no_map: &'a HashMap<String, i32>,
) -> View<G> {
    fn display_validate(validates: &[ValidateType], ty: ApiFieldType) -> Vec<String> {
        let validates: Vec<String> = validates
            .iter()
            .map(|v| {
                if let ValidateType::Enumer(descs) = v {
                    let mut result = String::default();
                    for s in descs {
                        if ty == ApiFieldType::Number {
                            result = format!("{}:{}; ", s.id, s.note);
                        } else {
                            result = format!("{}:{}; ", s.name, s.note);
                        }
                    }
                    result = format!("Enumer({})", result);
                    return result;
                }
                format!("{}", v)
            })
            .collect();
        validates
    }

    let mut field = member.serialize;
    if !serde {
        field = member.deserialize;
    }

    if let Some(field) = field {
        let href = field.inner.unwrap_or_default();
        let href = no_map
            .get(&href)
            .map(|i| format!("#{}", i))
            .unwrap_or_default();
        let href_signal = create_signal(cx, href.clone());
        let validates = display_validate(&member.validate, field.ty.clone());
        let note = if member.note.is_empty() {
            String::from("-")
        } else {
            member.note
        };
        let validate = if validates.is_empty() {
            String::from("-")
        } else {
            validates.join(" && ")
        };

        return view! {cx,
            tr {
                td {(field.name)}
                td {(field.ty) a(href=href_signal.get()) {(href)}}
                td {(field.option)}
                td {(validate)}
                td {(note)}
            }
        };
    }
    view! {cx,}
}

#[component(inline_props)]
pub fn ModelTable<'a, G: Html>(
    cx: Scope<'a>,
    model: ApiModel,
    serde: bool,
    index: i32,
    no_map: &'a HashMap<String, i32>,
) -> View<G> {
    let members = create_signal(cx, model.members);
    let tag = if index == -1 {
        match model.model_id {
            Some(model_id) => no_map
                .get(&model_id)
                .map(|i| format!("{}", i))
                .unwrap_or_default(),
            None => String::default(),
        }
    } else {
        format!("{}", index)
    };
    let tag_signal = create_signal(cx, tag.clone());
    view! {cx,
        div (id = tag_signal.get()) {span(class = "tag is-warning is-medium") {"#"(tag)}}
        table(class="table", style = "width: 100%;") {
            thead {
                tr {
                    th {abbr {"字段"}}
                    th {abbr {"类型"}}
                    th {abbr {"可选"}}
                    th {abbr {"限制"}}
                    th {abbr {"说明"}}
                }
            }
            tbody {
                Indexed (iterable = members,view= move |cx, item| view! { cx,
                    MemberRow(member = item, serde = serde, no_map = no_map)
                })
            }
        }
    }
}

#[component(inline_props)]
pub fn ApiTabTitle<'a, G: Html>(
    cx: Scope<'a>,
    api: &'a ApiOperation,
    selected: &'a Signal<String>,
) -> View<G> {
    let li_class = create_memo(cx, || {
        if api.url.eq(selected.get().as_str()) {
            return String::from("is-active");
        }
        String::default()
    });
    let close_icon_class = create_memo(cx, || {
        if api.url.eq(selected.get().as_str()) {
            return String::from("icon");
        }
        String::from("is-hidden")
    });
    view! {cx,
        li (class = li_class.get(), on:click = move |_| {
            let current = selected.get();
            if !api.url.eq(current.as_str()) {
                selected.set(api.url.to_string());
            }
        }) {
            a {
                (api.name)
                span(class = close_icon_class.get(), on:click = move |_| {
                    let current = selected.get();
                    selected.set(format!("{}{}", CLOSE_ID, current.as_str()));
                }) {
                    i (class = "fas fa-window-close") {}
                }
            }
        }
    }
}

#[component(inline_props)]
pub fn ParamTabTitle<'a, G: Html>(
    cx: Scope<'a>,
    title: &'a String,
    active: &'a Signal<String>,
    is_allowed: &'a Signal<bool>,
) -> View<G> {
    let class = create_memo(cx, || {
        let active = active.get();
        if active.as_ref().eq(title) {
            return String::from("is-active");
        }
        String::default()
    });
    let style = create_memo(cx, || {
        let is_allowed = is_allowed.get();
        if !*is_allowed.as_ref() {
            return String::from("cursor: not-allowed; color:grey;");
        }
        String::default()
    });
    let content = title.to_string();
    view! {cx,
        a (class = class.get(), style = style.get(), on:click = |_| {
            if *is_allowed.get().as_ref() {
                active.set(title.to_string())
            }
        }) {
            (content)
        }
    }
}

#[component(inline_props)]
pub fn ParamTabContent<'a, G: Html>(
    cx: Scope<'a>,
    title: &'a String,
    active: &'a Signal<String>,
    api: &'a ApiOperation,
) -> View<G> {
    let hide = create_memo(cx, || {
        let active = active.get();
        if !active.as_ref().eq(title) {
            return String::from("is-hidden");
        }
        String::from("")
    });
    let query = format!("{}", ParamTabType::Query);
    let path = format!("{}", ParamTabType::Path);
    let header = format!("{}", ParamTabType::Header);
    let body = format!("{}", ParamTabType::Body);
    if query.eq(title) || path.eq(title) || header.eq(title) {
        let models = if query.eq(title) {
            api.query_in.clone()
        } else if path.eq(title) {
            api.path_in.clone()
        } else {
            api.header_in.clone()
        };
        let model_no_map = create_ref(cx, HashMap::new());
        let models = create_signal(cx, models.into_iter().enumerate().collect());
        return view! {cx,
            Indexed(
                iterable=models,
                view=move |cx, (i, item)| {
                    view! {cx,
                        div (class = hide.get(), style = "width: 100%; margin-top: 10px;") {
                            ModelTable(model = item, index = i as i32, serde = false, no_map = model_no_map)
                        }
                    }
                }
            )
        };
    }

    fn model_no_map(model_id: &str, models: &HashMap<String, ApiModel>) -> HashMap<String, i32> {
        let mut model_no_map: HashMap<String, i32> = HashMap::new();
        let mut no = 1;
        for kv in models.iter() {
            if kv.0.eq(model_id) {
                continue;
            }
            model_no_map.insert(kv.0.to_string(), no);
            no += 1;
        }
        model_no_map.insert(model_id.to_string(), 0);
        model_no_map
    }

    let mut item = &api.body_in;
    if !body.eq(title) {
        item = &api.body_out;
    }

    if let Some(item) = item {
        let model_no_map = model_no_map(&item.model_id, &item.models);
        let model_no_map = create_ref(cx, model_no_map);
        let model = item.models.get(&item.model_id);

        let mut inners: Vec<ApiModel> = item.models.clone().into_values().collect();
        inners.retain(|x| match &x.model_id {
            Some(model_id) => !model_id.eq(&item.model_id),
            None => true,
        });
        let inners = create_signal(cx, inners);
        if let Some(model) = model {
            return view! {cx,
                div (class = hide.get(), style = "width: 100%;") {
                    span (class = "tag is-success is-light is-medium", style = "margin-bottom: 10px; border-left: 5px solid red;") {(item.content_type)}
                    ModelTable(model = model.clone(), index = -1, serde = false, no_map = model_no_map)
                    Indexed(
                        iterable=inners,
                        view=move |cx, item| {
                            view! {cx,
                                ModelTable(model = item, index = -1, serde = false, no_map = model_no_map)
                            }
                        }
                    )
                }
            };
        }
    }
    view! {cx,}
}

#[derive(Debug, Display, PartialEq, Eq)]
pub enum ParamTabType {
    Query,
    Path,
    Header,
    Body,
    Return,
}

#[component(inline_props)]
pub fn ApiBody<'a, G: Html>(cx: Scope<'a>, api: &'a ApiOperation) -> View<G> {
    let show_note = !api.note.is_empty();
    let active_panel = create_signal(cx, format!("{}", ParamTabType::Return));

    let query_title = create_ref(cx, format!("{}", ParamTabType::Query));
    let path_title = create_ref(cx, format!("{}", ParamTabType::Path));
    let header_title = create_ref(cx, format!("{}", ParamTabType::Header));
    let body_title = create_ref(cx, format!("{}", ParamTabType::Body));
    let return_title = create_ref(cx, format!("{}", ParamTabType::Return));

    let query_allow = create_signal(cx, !api.query_in.is_empty());
    let path_allow = create_signal(cx, !api.path_in.is_empty());
    let header_allow = create_signal(cx, !api.header_in.is_empty());
    let body_allow = create_signal(cx, api.body_in.is_some());
    let return_allow = create_signal(cx, api.body_out.is_some());

    let auth = create_signal(cx, api.auth);
    let auth_class = create_memo(cx, || {
        if !*auth.get().as_ref() {
            return String::from("is-hidden");
        }
        String::from("tag is-warning is-light is-medium")
    });

    view! {cx,
        div (class = "panel-heading has-background-link has-text-white") {
            span (class = "tag is-info is-light is-medium", style = "margin-right: 30px;"){
                (api.method)
            }
            span (class = auth_class.get(), style = "margin-right: 30px;"){
                "auth"
            }
            span {(api.url)}
        }
        (
            if show_note {
                view! {cx,
                    div {
                        span (class = "tag has-text-black is-danger is-light is-medium", style = "width: 100%;"){
                            span (class = "has-text-danger") {
                                "Note!"
                            }
                            (api.note)
                        }
                    }
                }
            } else {
                view! {cx,}
            }
        )
        p (class = "panel-tabs") {
            ParamTabTitle(title=query_title, active = active_panel, is_allowed = query_allow)
            ParamTabTitle(title=path_title, active = active_panel, is_allowed = path_allow)
            ParamTabTitle(title=header_title, active = active_panel, is_allowed = header_allow)
            ParamTabTitle(title=body_title, active = active_panel, is_allowed = body_allow)
            ParamTabTitle(title=return_title, active = active_panel, is_allowed = return_allow)
        }
        div (class = "panel-block") {
            ParamTabContent(title=query_title, active = active_panel, api = api)
            ParamTabContent(title=path_title, active = active_panel, api = api)
            ParamTabContent(title=header_title, active = active_panel, api = api)
            ParamTabContent(title=body_title, active = active_panel, api = api)
            ParamTabContent(title=return_title, active = active_panel, api = api)
        }
        // article (class = "panel is-link", style = "height: 100%;") {

        // }
    }
}

#[component(inline_props)]
pub fn TabBody<'a, G: Html>(
    cx: Scope<'a>,
    api: &'a ApiOperation,
    selected: &'a Signal<String>,
) -> View<G> {
    let class = create_memo(cx, || {
        let selected = selected.get();
        if !api.url.eq(selected.as_str()) {
            return String::from("is-hidden");
        }
        String::default()
    });
    view! {cx,
        div (class = class.get(), style = "height: 100%;") {
            ApiBody(api = api)
        }
    }
}

#[component]
pub fn ApiTabs<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    let li_class = create_memo(cx, || {
        if NOT_URL_ID.eq(props.selected.get().as_str()) {
            return String::from("is-active");
        }
        String::default()
    });

    let doc_tag_class = create_memo(cx, || {
        let selected = props.selected.get();
        if !selected.as_ref().eq(NOT_URL_ID) {
            return String::from("is-hidden");
        }
        String::default()
    });
    view! {cx,
        div (class = "tabs is-centered is-boxed", style = "width: 100%;") {
            ul{
                li (class = li_class.get(), on:click = move |_| {
                    let current = props.selected.get();
                    if !NOT_URL_ID.eq(current.as_str()) {
                        props.selected.set(NOT_URL_ID.to_string());
                    }
                }) {
                    a {
                        span {"文档说明"}
                    }
                }
                Indexed(
                    iterable=props.apis,
                    view=move |cx, item| {
                        let item = create_ref(cx, item);
                        view! {cx,
                            ApiTabTitle(api = item, selected = props.selected)
                        }
                    },
                )
            }
        }
        div (style = "width: 100%;") {
            Indexed(
                iterable=props.apis,
                view=move |cx, item| {
                    let item = create_ref(cx, item);
                    view! {cx,
                        TabBody(api=item, selected = props.selected)
                    }
                },
            )
        }
        div (class = doc_tag_class.get(), style = "width: 100%;") {
            h1 {(props.doc.name)}
            p {
                (props.doc.note)
            }
        }
    }
}
