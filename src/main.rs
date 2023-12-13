use leptos::*;

use enchanted_natures_web_app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    leptos::mount_to_body(|| view! { <App/> })
}
