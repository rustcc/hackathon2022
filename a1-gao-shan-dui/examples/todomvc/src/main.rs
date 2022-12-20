use akun::{
    view, DomNode, Event, GenericComponent, GenericElement, GenericNode, If, List, Scope, ScopeExt,
    Show, Signal,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, KeyboardEvent};

#[derive(Clone)]
struct Todo {
    content: Signal<String>,
    editing: Signal<bool>,
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
    let Todo {
        content,
        editing,
        completed,
        removed,
    } = todo.clone();
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
    let edit_input = cx.create_node_ref::<N>();
    let set_editing = move |_| {
        editing.set(true);
        if let Some(input) = edit_input.get::<DomNode>() {
            input
                .into_web_sys()
                .unchecked_into::<HtmlInputElement>()
                .focus()
                .unwrap_throw();
        }
    };
    let save_editing = move |ev: Event| {
        if editing.get() {
            let input = ev
                .current_target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            content.set(input.value());
            editing.set(false);
        }
    };
    let done_editing = move |ev: Event| {
        let ev = ev.unchecked_into::<web_sys::KeyboardEvent>();
        if ev.key() == "Enter" {
            save_editing(ev.into());
        }
    };
    view! { cx,
        li {
            .class("todo")
            .toggle_class("completed", completed)
            .toggle_class("editing", editing)
            *Show { *If {
                .when(show)
                [div {
                    .class("view")
                    input {
                        .class("toggle")
                        :type("checkbox")
                        :checked(completed)
                        @input(toggle)
                    }
                    label { @dblclick(set_editing) (content) }
                    button { .class("destroy") @click(remove) }
                }
                *Show { *If {
                    .when(editing)
                    input {
                        .class("edit")
                        .ref_(edit_input)
                        :value(content)
                        @blur(save_editing)
                        @keypress(done_editing)
                    }
                } }]
            } }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    akun::mount_to_body(|cx| {
        let todos = cx.create_signal(vec![]);
        let show_mode = cx.create_signal(ShowMode::All);

        let add_todo = move |ev: Event| {
            let ev = ev.unchecked_into::<KeyboardEvent>();
            if ev.key() == "Enter" {
                let input = ev
                    .current_target()
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>();
                let todo = Todo {
                    content: cx.create_signal(input.value().trim().to_owned()),
                    editing: cx.create_signal(false),
                    completed: cx.create_signal(false),
                    removed: cx.create_signal(false),
                };
                todos.update(|todos| todos.iter().cloned().chain(Some(todo)).collect())
            }
        };
        let make_todo = move |todo: &Todo| make_todo(cx, show_mode, todo);
        let remaining_count = cx.create_memo(move || {
            todos
                .get()
                .iter()
                .filter(|todo| !todo.removed.get())
                .count()
        });
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
                        @keypress(add_todo)
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
                        strong { (remaining_count) } " item"
                        (cx.create_memo(move || if remaining_count.get() > 1 { "s" } else { "" }))
                        " left"
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
                        .when(move || remaining_count.get() > 0)
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
