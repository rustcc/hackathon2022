/// [`view!`] 宏的内部实现
#[macro_export]
#[doc(hidden)]
macro_rules! __view_internal {
    // 元素
    // ----
    // div {
    //     .class("myClass")
    // }
    // => view(cx).root(|div: div| div.class("myClass"))
    (   cx=[$cx:ident]
        prefix=[]
        path=[$path:path]
        children=[$($children:tt)*]
        props=[$($props:tt)*]
        rest=[]
    ) => {
        $crate::__private::view_element(
            $cx,
            $path,
            move |__view| { __view $($props)* },
            move |__view| { __view $($children)* },
        )
    };
    // 组件
    // ----
    // *Counter {
    //     .init(233)
    // }
    // => Counter(cx).init(233).build()
    (   cx=[$cx:ident]
        prefix=[*]
        path=[$path:path]
        children=[$($children:tt)*]
        props=[$($props:tt)*]
        rest=[]
    ) => {
        $crate::__private::view_component(
            $cx,
            $path,
            move |__view| { __view $($props)* },
            move |__view| { __view $($children)* },
            move |__view| { __view .build() },
        )
    };
    // 解析 Properties 函数
    // -----------------
    // div {
    //     .class("myClass")
    // }
    // => .class("myClass")
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=$children:tt
        props=[$($props:tt)*]
        rest=[.$method:ident $args:tt $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=$children
            props=[$($props)* .$method $args]
            rest=[$($rest)*]
        }
    };
    // input {
    //     :type("button")
    // }
    // => .prop("type", "button")
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=$children:tt
        props=[$($props:tt)*]
        rest=[:$prop:ident ($val:expr $(,)?) $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=$children
            props=[$($props)* .prop(stringify!($prop), $val)]
            rest=[$($rest)*]
        }
    };
    // input {
    //     @change(handler)
    // }
    // => .on("change", handler)
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=$children:tt
        props=[$($props:tt)*]
        rest=[@$event:ident ($handler:expr $(,)?) $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=$children
            props=[$($props)* .on(stringify!($event), $handler)]
            rest=[$($rest)*]
        }
    };
    // 解析 Children 函数
    // -----------------
    // input {
    //     "some text"
    // }
    // => .child(view!(cx, text { .data("some text") }))
    (   cx=[$cx:ident]
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[$text:literal $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=[$cx]
            prefix=$prefix
            path=$path
            children=[
                $($children)*
                .child($crate::__private::view_text($cx, $text))
            ]
            props=$props
            rest=[$($rest)*]
        }
    };
    // input {
    //     ("more text")
    // }
    // => .child(view!(cx, text { .data("some text") }))
    (   cx=[$cx:ident]
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[($text:expr) $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=[$cx]
            prefix=$prefix
            path=$path
            children=[
                $($children)*
                .child($crate::__private::view_text($cx, $text))
            ]
            props=$props
            rest=[$($rest)*]
        }
    };
    // input {
    //     div { ... }
    // }
    // => .child(view!(cx, div { ... }))
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[$child_name:ident {$($child_args:tt)*} $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=[
                $($children)*
                .child($crate::__view_internal! {
                    cx=$cx
                    prefix=[]
                    path=[$crate::elements::$child_name]
                    children=[]
                    props=[]
                    rest=[$($child_args)*]
                })
            ]
            props=$props
            rest=[$($rest)*]
        }
    };
    // input {
    //     *Counter { }
    // }
    // => .child(Counter(cx).build())
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[* $child_name:ident {$($child_args:tt)*} $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=[
                $($children)*
                .child($crate::__view_internal! {
                    cx=$cx
                    prefix=[*]
                    path=[$child_name]
                    children=[]
                    props=[]
                    rest=[$($child_args)*]
                })
            ]
            props=$props
            rest=[$($rest)*]
        }
    };
    // input {
    //     [...]
    // }
    // => .child(Fragment(cx).child(...))
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[[$($fragment:tt)*] $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=[
                $($children)*
                .child($crate::__view_internal! {
                    cx=$cx
                    prefix=[*]
                    path=[$crate::components::Fragment]
                    children=[]
                    props=[]
                    rest=[$($fragment)*]
                })
            ]
            props=$props
            rest=[$($rest)*]
        }
    };
    // input {
    //     {expr_child}
    // }
    // => .child(expr_child)
    (   cx=$cx:tt
        prefix=$prefix:tt
        path=$path:tt
        children=[$($children:tt)*]
        props=$props:tt
        rest=[{$child:expr} $($rest:tt)*]
    ) => {
        $crate::__view_internal! {
            cx=$cx
            prefix=$prefix
            path=$path
            children=[$($children)* .child($child)]
            props=$props
            rest=[$($rest)*]
        }
    };
}

/// 生成一个唯一的 [`TemplateId`]，并使用当前模块路径以及行列号作为附加调试信息。
///
/// [`TemplateId`]: crate::template::TemplateId
#[macro_export]
macro_rules! id {
    () => {{
        thread_local! {
            static __ID: $crate::template::TemplateId =
                $crate::template::TemplateId::generate(concat!(module_path!(), ":", line!(), ":", column!()));
        }
        __ID.with($crate::template::TemplateId::clone)
    }};
}

/// 便于声明 UI 的辅助宏，具体用例请参考 `akun::macros::tests`。
// TODO: 我们可以使用一个 proc_macro 来分辨是否为元素或者组件
#[macro_export]
macro_rules! view {
    ($cx:expr, $($args:tt)*) => {{
        fn __id() -> $crate::template::TemplateId {
            $crate::id!()
        }
        let __cx = $cx;
        $crate::__view_internal! {
            cx=[__cx]
            prefix=[*]
            path=[$crate::__private::ViewRoot]
            children=[]
            props=[.id(__id)]
            rest=[$($args)*]
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::{create_root, DomNode, GenericComponent, GenericNode, Scope};
    use std::marker::PhantomData;

    struct AwesomeComponent<N> {
        cx: Scope,
        marker: PhantomData<N>,
    }

    impl<N: GenericNode> AwesomeComponent<N> {
        fn build(self) -> impl GenericComponent<N> {
            view! { self.cx, "Awesome!" }
        }
    }

    #[allow(non_snake_case)]
    fn AwesomeComponent<N: GenericNode>(cx: Scope) -> AwesomeComponent<N> {
        AwesomeComponent {
            cx,
            marker: PhantomData,
        }
    }

    fn method<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, div { .class("myClass") } }
    }

    fn property<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, input { :type("button") } }
    }

    fn event<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, input { @change(|_| {}) } }
    }

    fn text_literal<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, "233" }
    }

    fn text_expr<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, ("233") }
    }

    fn element<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, div { } }
    }

    fn component<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, *AwesomeComponent { } }
    }

    fn fragment<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, [ "some" "texts" ] }
    }

    fn child_expr<N: GenericNode>(cx: Scope) -> impl GenericComponent<N> {
        view! { cx, { view! { cx, [ "other" "texts" ] } } }
    }

    #[test]
    fn macros() {
        create_root(|cx| {
            macro_rules! run_tests {
                ($($name:ident),*$(,)?) => {$(
                    let _ = $name::<DomNode>(cx);
                )*};
            }

            run_tests!(
                method,
                property,
                event,
                text_literal,
                text_expr,
                element,
                component,
                fragment,
                child_expr,
            );
        });
    }
}
