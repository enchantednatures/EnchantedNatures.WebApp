use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn horizontal_scroll(el: web_sys::Element) {
    let html_el = el.dyn_ref::<web_sys::HtmlElement>().unwrap().clone();
    let closure = Closure::wrap(Box::new(move |e: web_sys::WheelEvent| {
        e.prevent_default();
        html_el.set_scroll_left(html_el.scroll_left() + e.delta_y() as i32);
    }) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
