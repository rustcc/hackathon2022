#[path = "elements/text.rs"]
mod text_;
pub use text_::text;

use crate::{EventHandler, GenericElement, GenericNode, NodeType, Scope, Signal};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

type StaticStr = &'static str;

/// 一个可能为响应性，即动态更新的值，例如 [`Signal`]。
///
/// [`Signal`]: crate::Signal
#[derive(Clone)]
pub enum Reactive<T> {
    Value(T),
    Fn(Rc<dyn Fn() -> Reactive<T>>),
}

/// [`Into`] 的别名，便于接收一个响应性的值作为参数。
pub trait IntoReactive<T>: Into<Reactive<T>> {
    fn into_reactive(self) -> Reactive<T> {
        self.into()
    }

    /// 读取当前值，立即执行响应函数。
    fn into_value(self) -> T {
        let mut current = self.into_reactive();
        loop {
            match current {
                Reactive::Value(t) => return t,
                Reactive::Fn(f) => current = f(),
            }
        }
    }
}

impl<T, U: Into<Reactive<T>>> IntoReactive<T> for U {}

impl<T, F, U> From<F> for Reactive<T>
where
    F: 'static + Fn() -> U,
    U: IntoReactive<T>,
{
    fn from(t: F) -> Self {
        Reactive::Fn(Rc::new(move || (t)().into()))
    }
}

impl<T, U> From<Signal<U>> for Reactive<T>
where
    U: Clone + IntoReactive<T>,
{
    fn from(t: Signal<U>) -> Self {
        Reactive::Fn(Rc::new(move || t.get().into()))
    }
}

/// 为一些基本类型实现 [`IntoReactive`]。
macro_rules! impl_into_reactive {
    ($($ty:ident),*$(,)?) => {$(
        impl From<$ty> for Reactive<$ty> {
            fn from(t: $ty) -> Self {
                Reactive::Value(t)
            }
        }
    )*};
}

impl_into_reactive!(
    bool, i8, u8, i16, u16, char, i32, u32, i64, u64, isize, usize, i128, u128, String
);

/// 为一些泛型类型实现 [`IntoReactive`]。
macro_rules! impl_into_reactive_generic {
    ($($ty:ident),*$(,)?) => {$(
        impl<T> From<$ty<T>> for Reactive<$ty<T>> {
            fn from(t: $ty<T>) -> Self {
                Reactive::Value(t)
            }
        }
    )*};
}

impl_into_reactive_generic!(Rc, Option, Vec, RefCell, Cell);

impl From<&str> for Reactive<String> {
    fn from(t: &str) -> Self {
        Reactive::Value(t.to_owned())
    }
}

/// [`Into`] 的别名，便于接受一个事件处理者作为参数。
pub trait IntoEventHandler: Into<EventHandler> {
    fn into_event_handler(self) -> EventHandler {
        self.into()
    }
}

impl<T: Into<EventHandler>> IntoEventHandler for T {}

impl<F> From<F> for EventHandler
where
    F: 'static + FnMut(web_sys::Event),
{
    fn from(f: F) -> Self {
        EventHandler {
            handler: Box::new(f),
        }
    }
}

/// 定义一些基本 HTML 元素。
macro_rules! define_elements {
    ($($tag:ident),*$(,)?) => {$(
        #[doc = concat!("`", stringify!($tag), "` HMTL 元素")]
        #[allow(non_camel_case_types)]
        pub struct $tag<N> {
            cx: Scope,
            node: N,
        }

        #[doc = concat!("创建一个 [`struct@", stringify!($tag), "`] 元素")]
        pub fn $tag<N: GenericNode>(cx: Scope) -> $tag<N> {
            GenericElement::create(cx)
        }

        impl<N: GenericNode> GenericElement<N> for $tag<N> {
            const TYPE: NodeType = NodeType::Tag(stringify!($tag));
            fn create_with_node(cx: Scope, node: N) -> Self {
                Self { cx, node }
            }
            fn into_node(self) -> N {
                self.node
            }
        }

        impl<N: GenericNode> $tag<N> {
            /// 设定 `property`，`val` 的值将会被跟踪并动态更新。
            pub fn prop<V: IntoReactive<String>>(self, name: StaticStr, val: V) -> Self {
                let node = self.node.clone();
                let val = val.into_reactive();
                self.cx.create_effect(move || {
                    node.set_property(name, &val.clone().into_value());
                });
                self
            }

            /// 设定 `attribute`，`val` 的值将会被跟踪并动态更新。
            pub fn attr<V: IntoReactive<String>>(self, name: StaticStr, val: V) -> Self {
                let node = self.node.clone();
                let val = val.into_reactive();
                self.cx.create_effect(move || {
                    node.set_attribute(name, &val.clone().into_value());
                });
                self
            }

            /// 监听一个事件。
            pub fn on<E: IntoEventHandler>(self, event: StaticStr, handler: E) -> Self {
                self.node.listen_event(event, handler.into_event_handler());
                self
            }

            /// 添加一个 `class`。
            pub fn class(self, name:  StaticStr) -> Self {
                self.node.add_class(name);
                self
            }

            /// 动态更新 `class`，当 `toggle` 为 `true` 时，添加该 `class`，否则移除。
            pub fn toggle_class<V: IntoReactive<bool>>(self, name:  StaticStr, toggle: V) -> Self {
                let node = self.node.clone();
                let toggle = toggle.into_reactive();
                self.cx.create_effect(move || {
                    if toggle.clone().into_value() {
                        node.add_class(name);
                    } else {
                        node.remove_class(name);
                    }
                });
                self
            }
        }
    )*};
}

define_elements!(
    a, abbr, address, area, article, aside, audio, b, base, bdi, bdo, blockquote, body, br, button,
    canvas, caption, cite, code, col, colgroup, data, datalist, dd, del, details, dfn, dialog, div,
    dl, dt, em, embed, fieldset, figcaption, figure, footer, form, head, header, hgroup, h1, h2,
    h3, h4, h5, h6, hr, html, i, iframe, img, input, ins, kbd, keygen, label, legend, li, link,
    main, map, mark, menu, menuitem, meta, meter, nav, noscript, object, ol, optgroup, option,
    output, p, param, picture, pre, progress, q, rp, rt, ruby, s, samp, script, section, select,
    small, source, span, strong, style, sub, summary, sup, table, tbody, td, template, textarea,
    tfoot, th, thead, time, title, tr, track, u, ul, var, video, wbr,
);
