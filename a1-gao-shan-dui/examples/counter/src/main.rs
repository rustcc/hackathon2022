use akun::view;

fn main() {
    console_error_panic_hook::set_once();

    akun::render_to_body(|cx| {
        let counter = cx.create_signal(0);
        let increment = move |_| counter.update(|x| *x + 1);
        view! { cx,
            div {
                button {
                    @click(increment)
                    "Click me: " (counter) " times!"
                }
            }
        }
    });
}
