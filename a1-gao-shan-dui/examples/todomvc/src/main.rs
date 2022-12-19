use akun::{view, GenericComponent, GenericNode, If, List, Scope, Show, Signal};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};

#[derive(Clone)]
struct Todo {
    content: String,
    completed: Signal<bool>,
    removed: Signal<bool>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ShowMode {
    All,
    Active,
    Completed,
}

fn make_todo<N: GenericNode>(
    cx: Scope,
    show_mode: Signal<ShowMode>,
    todo: &Todo,
) -> impl GenericComponent<N> {
    let completed = todo.completed;
    let removed = todo.removed;
    let content = todo.content.clone();
    let toggle = move |_| completed.update(|x| !*x);
    let remove = move |_| removed.set(true);
    let show = move || {
        if removed.get() {
            false
        } else {
            match show_mode.get() {
                ShowMode::All => true,
                ShowMode::Active => !completed.get(),
                ShowMode::Completed => completed.get(),
            }
        }
    };
    view! { cx,
        li {
            .class("todo")
            .toggle_class("completed", completed)
            *Show { *If {
                .when(show)
                div {
                    .class("view")
                    input {
                        .class("toggle")
                        :type("checkbox")
                        :checked(completed)
                        @input(toggle)
                    }
                    label { (content) }
                    button { .class("destroy") @click(remove) }
                    // TODO: 添加 edit 功能
                }
            } }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    akun::mount_to_body(|cx| {
        let todos = cx.create_signal(vec![]);
        let show_mode = cx.create_signal(ShowMode::All);

        let add_todo = move |ev: akun::Event| {
            let ev = ev.unchecked_into::<KeyboardEvent>();
            if ev.key() == "Enter" {
                let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                let todo = Todo {
                    content: input.value().trim().to_owned(),
                    completed: cx.create_signal(false),
                    removed: cx.create_signal(false),
                };
                todos.update(|todos| todos.iter().cloned().chain(Some(todo)).collect())
            }
        };
        let make_todo = move |todo: &Todo| make_todo(cx, show_mode, todo);
        let remaining_count = move || {
            todos
                .get()
                .iter()
                .filter(|todo| !todo.removed.get())
                .count()
        };
        let filter_selected = move |mode: ShowMode| move || show_mode.get() == mode;
        let filter_set = move |mode: ShowMode| move |_: akun::Event| show_mode.set(mode);
        let clear_complted = move |_| {
            todos.get().iter().for_each(|todo| {
                if todo.completed.get() {
                    todo.removed.set(true);
                }
            })
        };

        view! { cx,
            section {
                .class("todoapp")
                header {
                    .class("header")
                    h1 { "Todos" }
                    input {
                        .class("new-todo")
                        :placeholder("What needs to be done?")
                        @keydown(add_todo)
                    }
                }
                *Show { *If {
                    .when(move || !todos.get().is_empty())
                    section {
                        .class("main")
                        ul {
                            .class("todo-list")
                            *List { .each(todos) {make_todo} }
                        }
                    }
                } }
                footer {
                    .class("footer")
                    span {
                        .class("todo-count")
                        strong { (remaining_count) } " item(s) left"
                    }
                    ul {
                        .class("filters")
                        li { a {
                            :href("#/") "All"
                            .toggle_class("selected", filter_selected(ShowMode::All))
                            @click(filter_set(ShowMode::All))
                        } }
                        li { a {
                            :href("#/active") "Active"
                            .toggle_class("selected", filter_selected(ShowMode::Active))
                            @click(filter_set(ShowMode::Active))
                        } }
                        li { a {
                            :href("#/completed") "Completed"
                            .toggle_class("selected", filter_selected(ShowMode::Completed))
                            @click(filter_set(ShowMode::Completed))
                        } }
                    }
                    *Show { *If {
                        .when(move || remaining_count() > 0)
                        button {
                            .class("clear-completed")
                            @click(clear_complted)
                            "Clear completed"
                        }
                    } }
                }
            }
        }
    });
}
