use akun::{view, Else, If, Show};

fn main() {
    console_error_panic_hook::set_once();

    akun::mount_to_body(|cx| {
        let counter = cx.create_signal(0);
        let increment = move |_| counter.update(|x| *x + 1);
        let is_even = move || counter.get() % 2 == 0;
        view! { cx,
            div {
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
    });
}
