use crate::{node::UnwrapThrowValExt, EventHandler, DOCUMENT};
use js_sys::{Function, Object, Reflect};
use std::{borrow::Cow, cell::RefCell, collections::HashMap};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{Event, Node};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    type Descriptor;

    #[wasm_bindgen(method, setter)]
    fn set_configurable(this: &Descriptor, val: bool);

    #[wasm_bindgen(method, setter, js_name = value)]
    fn set_target_value(this: &Descriptor, val: &JsValue);

    #[wasm_bindgen(method, setter, js_name = get)]
    fn set_current_target_get(this: &Descriptor, val: &Function);

    #[wasm_bindgen(extends = Node)]
    #[derive(Clone, PartialEq, Eq)]
    type EvTarget;

    #[wasm_bindgen(method, getter, js_name = "$$$EVENT_HANDLERS")]
    fn event_handlers(this: &EvTarget) -> Option<Object>;

    #[wasm_bindgen(method, setter, js_name = "$$$EVENT_HANDLERS")]
    fn set_event_handlers(this: &EvTarget, val: &Object);

    #[wasm_bindgen(method, getter)]
    fn disabled(this: &EvTarget) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = parentNode)]
    fn parent(this: &EvTarget) -> Option<EvTarget>;

    #[wasm_bindgen(method, getter)]
    fn host(this: &EvTarget) -> Option<EvTarget>;

    #[wasm_bindgen(extends = Object)]
    #[derive(Clone)]
    type SharedNode;

    #[wasm_bindgen(method, getter)]
    fn inner(this: &SharedNode) -> Option<Node>;

    #[wasm_bindgen(method, setter)]
    fn set_inner(this: &SharedNode, inner: &Node);
}

type DelegatedEvents = HashMap<&'static str, bool>;

thread_local! {
    static GLOBAL_EVENTS: RefCell<DelegatedEvents> = RefCell::new(delegated_events());
}

fn delegated_events() -> DelegatedEvents {
    [
        "beforeinput",
        "click",
        "contextmenu",
        "dblclick",
        "focusin",
        "focusout",
        "input",
        "keydown",
        "keyup",
        "mousedown",
        "mousemove",
        "mouseout",
        "mouseover",
        "mouseup",
        "pointerdown",
        "pointermove",
        "pointerout",
        "pointerover",
        "pointerup",
        "touchend",
        "touchmove",
        "touchstart",
    ]
    .into_iter()
    .map(|key| (key, false))
    .collect()
}

fn reverse_shadow_dom_retargetting(ev: &Event, target: &EvTarget) {
    thread_local! {
        static K_TARGET: JsValue = JsValue::from_str("target");
    }

    let descriptor = Object::new().unchecked_into::<Descriptor>();
    descriptor.set_configurable(true);
    descriptor.set_target_value(target);
    K_TARGET.with(|key| Object::define_property(ev, key, &descriptor));
}

fn simulate_current_target(ev: &Event, node: &SharedNode) {
    thread_local! {
        static K_CURRENT_TARGET: JsValue = JsValue::from_str("currentTarget");
    }

    let node = node.clone();
    let descriptor = Object::new().unchecked_into::<Descriptor>();
    descriptor.set_configurable(true);
    descriptor.set_current_target_get(
        &Closure::<dyn FnMut() -> Option<Node>>::new(move || node.inner())
            .into_js_value()
            .unchecked_into(),
    );
    K_CURRENT_TARGET.with(|key| Object::define_property(ev, key, &descriptor));
}

fn event_handler(name: JsValue) -> Function {
    let handler = move |ev: Event| {
        let target: EvTarget = ev.target().unwrap().unchecked_into();
        let node: EvTarget = {
            let node = ev.composed_path().get(0);
            if node.is_undefined() {
                target.clone()
            } else {
                node.unchecked_into()
            }
        };
        // 反转 Shadow DOM 的 target 重定向
        if target != node {
            reverse_shadow_dom_retargetting(&ev, &node);
        }
        // 模拟 currentTarget
        let shared_node = Object::new().unchecked_into::<SharedNode>();
        simulate_current_target(&ev, &shared_node);
        // 模拟事件冒泡
        let mut current = node;
        loop {
            shared_node.set_inner(&current);
            if current.disabled().is_falsy() {
                if let Some(handlers) = current.event_handlers() {
                    let handler = Reflect::get(&handlers, &name).unwrap_throw_val();
                    if !handler.is_undefined() {
                        handler
                            .unchecked_into::<Function>()
                            .call1(&current, &ev)
                            .unwrap_throw_val();
                    }
                }
            }
            if ev.cancel_bubble() {
                break;
            }
            if let Some(next) = current.parent().or_else(|| current.host()) {
                current = next;
            } else {
                DOCUMENT.with(|doc| shared_node.set_inner(doc));
                break;
            }
        }
    };

    Closure::<dyn FnMut(Event)>::new(handler)
        .into_js_value()
        .unchecked_into()
}

pub fn add_event_listener(node: &Node, key: Cow<'static, str>, handler: EventHandler) {
    let handler = Closure::wrap(handler.handler)
        .into_js_value()
        .unchecked_into::<Function>();
    GLOBAL_EVENTS.with(|events| {
        let mut events = events.borrow_mut();
        if let Some(registered) = events.get_mut(&*key) {
            // 注入事件回调
            let name = JsValue::from_str(&key);
            let target = node.unchecked_ref::<EvTarget>();
            let handlers = target.event_handlers().unwrap_or_else(|| {
                let new = Object::new().unchecked_into();
                target.set_event_handlers(&new);
                new
            });
            Reflect::set(&handlers, &name, &handler).unwrap_throw_val();

            // 注册全局事件处理
            if !*registered {
                *registered = true;
                DOCUMENT.with(|doc| {
                    doc.add_event_listener_with_callback(&key, &event_handler(name))
                        .unwrap_throw_val();
                });
            }
        } else {
            // 不使用事件委托
            node.add_event_listener_with_callback(&key, &handler)
                .unwrap_throw_val();
        }
    });
}
