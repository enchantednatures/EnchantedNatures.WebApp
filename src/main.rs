use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct CategoryViewModel {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PhotoViewModel {
    pub id: i32,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub cloudflare_resource: String,
}

async fn get_photos(category_id: usize) -> Vec<PhotoViewModel> {
    let url = format!(
        "https://enchantednatures.shuttleapp.rs/api/v0/categories/{}",
        category_id
    );
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let photos: Vec<PhotoViewModel> = resp.json().await.unwrap();
    dbg!(&photos);
    photos
}

async fn get_categories() -> Vec<CategoryViewModel> {
    let url = "https://enchantednatures.shuttleapp.rs/api/v0/categories";
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let categories: Vec<CategoryViewModel> = resp.json().await.unwrap();
    dbg!(&categories);
    categories
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct GalleryParams {
    id: usize,
}

#[component]
fn Gallery() -> impl IntoView {
    let params = use_params::<GalleryParams>();
    let photos = create_resource(
        move || params().map(|params| params.id).ok().expect("no id"),
        get_photos,
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {photos
                .get()
                .unwrap_or(vec![])
                .into_iter()
                .map(|photo| {
                    view! {
                        <div class="gallery-body">
                            <a href="/">
                                <img
                                    alt=photo.title
                                    class="gallery-image"
                                    src=format!(
                                        "https://imagedelivery.net/ubbJTaQO-oc8x-62MYWhKg/{}/public",
                                        photo.cloudflare_resource,
                                    )
                                />

                            </a>
                        </div>
                    }
                })
                .collect_view()}
        </Transition>
    }
}

#[component]
fn App() -> impl IntoView {
    let categories_resource = create_resource(|| (), |_| async move { get_categories().await });
    let categories = move || categories_resource
        .get()
        .unwrap_or(vec![])
        .into_iter()
        .map(|category| {
            view! {
                <li class="controller-link">
                    <A href=format!("/category/{}", category.id)>{category.name}</A>
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="container-fluid">
            <div class="row">
                <Router>
                    <nav class="col-md-3 col-lg-2 bg-white sidebar">
                        <div class="sidebar-sticky pt-3">
                            <ul class="nav flex-column">
                                <li class="controller-link">
                                    <a>enchanted natures</a>
                                </li>
                                <Suspense fallback=move || {
                                    view! { <p>"Loading..."</p> }
                                }>

                                    {move || categories}
                                </Suspense>
                            </ul>
                        </div>
                    </nav>
                    <main>
                        <Route path="" view=|| view! { <Outlet/> }/>
                        <Route path="category" view=|| view! { <Outlet/> }>
                            <Route path=":id" view=|| view! { <Gallery/> }/>
                        </Route>
                    </main>
                </Router>
                <router-outlet></router-outlet>
            </div>
        </div>
    }
}
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
