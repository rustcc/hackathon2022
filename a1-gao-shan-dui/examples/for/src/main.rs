use akun::{view, For, If, Show};
use rand::seq::SliceRandom;
use std::cell::Cell;
use wasm_bindgen::JsValue;
use web_sys::console;

thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

fn new_id() -> usize {
    COUNTER.with(|id| {
        let current = id.get();
        id.set(current + 1);
        current
    })
}

fn main() {
    console_error_panic_hook::set_once();

    let mut rng = rand::thread_rng();
    akun::mount_to_body(|cx| {
        let ids = cx.create_signal(vec![]);
        cx.create_effect(move || {
            let arr = ids
                .get()
                .iter()
                .map(|&t| JsValue::from_f64(t as f64))
                .collect::<js_sys::Array>();
            console::log_1(&arr);
        });
        let show = cx.create_signal(true);
        let insert = move |_| {
            ids.update(|x| {
                let mut x = x.clone();
                let i = if x.is_empty() {
                    0
                } else {
                    rand::random::<usize>() % x.len()
                };
                x.insert(i, new_id());
                x
            });
        };
        let remove = move |_| {
            ids.update(|x| {
                let mut x = x.clone();
                if !x.is_empty() {
                    let i = rand::random::<usize>() % x.len();
                    x.remove(i);
                }
                x
            })
        };
        let shuffle = move |_| {
            ids.update(|x| {
                let mut x = x.clone();
                x.shuffle(&mut rng);
                x
            })
        };
        let clear = move |_| ids.update(|_| Default::default());
        let toggle = move |_| show.update(|x| !*x);
        view! { cx,
            div {
                div {
                    button { @click(insert) "Insert" }
                    button { @click(remove) "Remove" }
                    button { @click(shuffle) "Shuffle" }
                    button { @click(clear) "Clear" }
                    button { @click(toggle) "Toggle" }
                }
                *Show {
                    *If {
                        .when(show)
                        *For {
                            .each(ids)
                            .key(|v| *v)
                            {move |&id| view! { cx, fieldset { "ID: " (id.to_string()) } }}
                        }
                    }
                }
            }
        }
    });
}
