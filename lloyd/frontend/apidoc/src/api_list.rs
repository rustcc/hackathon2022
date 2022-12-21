use crate::atom::navigation::menu::{Item, NavMenu};
use common::doc::ApiOperation;
use sycamore::prelude::*;

pub fn nest_api_list(api_items: &mut Vec<Item>, index: usize, mod_items: &Vec<&str>, url: &str) {
    if index >= mod_items.len() {
        return;
    }
    let name = mod_items[index];
    for item in api_items.iter_mut() {
        if item.name.eq(name) {
            nest_api_list(&mut item.childs, index + 1, mod_items, url);
            item.amount += 1;
            return;
        }
    }
    let mut id = None;
    if index == mod_items.len() - 1 {
        id = Some(url.to_string());
    }
    let mut item = Item {
        id,
        name: name.to_string(),
        active: false,
        childs: vec![],
        amount: 1,
    };
    nest_api_list(&mut item.childs, index + 1, mod_items, url);
    api_items.push(item);
}

#[component(inline_props)]
pub async fn ApiList<'a, G: Html>(
    cx: Scope<'a>,
    apis: Vec<ApiOperation>,
    selected: &'a Signal<String>,
) -> View<G> {
    let mut api_items: Vec<Item> = vec![];
    for api in apis {
        let mut mod_items: Vec<&str> = api.mod_path.split("::").collect();
        mod_items.push(&api.name);
        nest_api_list(&mut api_items, 0, &mod_items, &api.url);
    }
    view! { cx,
        NavMenu(items = api_items, selected = selected)
    }
}
