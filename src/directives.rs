use leptos::ev::wheel;
use leptos::html::AnyElement;
use leptos::logging;
use leptos::HtmlElement;

pub fn horizontal_scroll(el: HtmlElement<AnyElement>) {
    _ = el.clone().on(wheel, move |e| {
        logging::log!("scrolling");
        e.prevent_default();
        el.set_scroll_left(el.scroll_left() + e.delta_y() as i32);
    });
}
