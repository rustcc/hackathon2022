use akun::{view, Else, GenericComponent, GenericNode, If, List, Scope, Show};

fn make_counter<N: GenericNode>(cx: Scope, initial: usize) -> impl GenericComponent<N> {
    let counter = cx.create_signal(initial);
    let increment = move |_| counter.update(|x| *x + 1);
    let is_even = move || counter.get() % 2 == 0;
    view! { cx,
        fieldset {
            div {
                "Number " (counter) " is "
                *Show {
                    *If { .when(is_even) "even" }
                    *Else { "odd" }
                }
                "."
            }
            button {
                @click(increment)
                "Click me: " (counter) " times!"
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    akun::mount_to_body(|cx| {
        let counters = cx.create_signal(vec![1, 2, 3, 4]);
        let show = cx.create_signal(true);
        let append =
            move |_| counters.update(|x| x.iter().cloned().chain(Some(x.len() + 1)).collect());
        let remove = move |_| {
            counters.update(|x| {
                if x.is_empty() {
                    Default::default()
                } else {
                    x.iter().cloned().take(x.len() - 1).collect()
                }
            })
        };
        let clear = move |_| counters.update(|_| Default::default());
        let toggle = move |_| show.update(|x| !*x);
        view! { cx,
            div {
                div {
                    button { @click(append) "Append" }
                    button { @click(remove) "Remove" }
                    button { @click(clear) "Clear" }
                    button { @click(toggle) "Toggle" }
                }
                *Show {
                    *If {
                        .when(show)
                        *List { .each(counters) {move |&init| make_counter(cx, init)} }
                    }
                }
            }
        }
    });
}
