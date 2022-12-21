use std::collections::HashMap;

use common::doc::{ApiDoc, ApiOperation};
use reqwasm::{http::Request, Error};
use sycamore::{prelude::*, suspense::Suspense};

use crate::{
    api_detail::{ApiNote, ApiTabs, CLOSE_ID, NOT_URL_ID},
    api_list::ApiList,
};

pub mod api_detail;
pub mod api_list;
pub mod atom;

pub async fn fetch_doc() -> Result<ApiDoc, Error> {
    let result = Request::get("http://127.0.0.1:9001/doc").send().await?;
    let result = result.json::<ApiDoc>().await?;
    Ok(result)
}

#[component]
pub async fn ShowApiDoc<'a, G: Html>(cx: Scope<'_>) -> View<G> {
    let apidoc = fetch_doc().await.unwrap();
    let api_note = ApiNote {
        name: apidoc.name.to_string(),
        note: apidoc.note.to_string(),
    };
    let apis = apidoc.apis;
    let mut api_map: HashMap<String, ApiOperation> = HashMap::new();
    for api in apis.iter() {
        api_map.insert(api.url.clone(), api.clone());
    }
    let selected = create_signal(cx, String::from(NOT_URL_ID));
    let opened_apis: &Signal<Vec<ApiOperation>> = create_signal(cx, vec![]);
    create_effect(cx, move || {
        let selected_rc = selected.get();
        if selected_rc.is_empty() {
            return;
        }
        let selected_str = selected_rc.as_str();
        if !selected_str.starts_with(CLOSE_ID) {
            for item in opened_apis.get().iter() {
                if item.url.eq(selected_str) {
                    return;
                }
            }
            let mut new_opened_apis = opened_apis.get().as_ref().clone();
            let new_api = api_map.get(selected_str);
            if let Some(new_api) = new_api {
                new_opened_apis.push(new_api.clone());
                opened_apis.set(new_opened_apis);
            }
            return;
        }

        if let Some(selected_str) = selected_str.strip_prefix(CLOSE_ID) {
            let opened_apis_rc = opened_apis.get();
            let mut new_opened_apis = opened_apis_rc.as_ref().clone();
            let mut remove_index = opened_apis_rc.len();
            for index in 0..opened_apis_rc.len() {
                let item = opened_apis_rc.get(index);
                if let Some(item) = item {
                    if item.url.eq(selected_str) {
                        remove_index = index;
                    }
                }
            }
            if remove_index != opened_apis_rc.len() {
                new_opened_apis.remove(remove_index);
            }
            if new_opened_apis.is_empty() {
                selected.set(NOT_URL_ID.into());
            } else if remove_index < new_opened_apis.len() {
                let next_selected = new_opened_apis.get(remove_index);
                if let Some(next_selected) = next_selected {
                    selected.set(next_selected.url.clone());
                }
            } else {
                let next_selected = new_opened_apis.get(remove_index - 1);
                if let Some(next_selected) = next_selected {
                    selected.set(next_selected.url.clone());
                }
            }
            opened_apis.set(new_opened_apis);
        }
    });

    view! { cx,
        div (class = "tile", style = "width: 100%;height: 100%; padding: 10px") {
            div (class = "tile is-2", style = "height: 100%; margin-right: 5px;") {
                div (class = "box", style = "width: 100%;height: 100%; overflow-y: auto;") {
                    ApiList(apis = apis, selected = selected)
                }
            }
            div (class = "tile", style = "width: 100%; height: 100%;") {
                div (class = "box", style = "margin: 0px; width: 100%; height: 100%; overflow-y: auto;") {
                    ApiTabs(apis = opened_apis, selected = selected, doc=api_note)
                }
            }
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let loading_view = view! {cx,
        "Loading..."
    };

    view! { cx,
        Suspense(fallback=loading_view) {
            ShowApiDoc
        }
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(|cx| view! { cx, App {} });
}
