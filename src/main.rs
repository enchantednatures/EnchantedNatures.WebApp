use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use reqwest::Client;
use enchanted_natures_web_app::App;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
