use leptos::*;
use leptos_router::*;
mod api;
use api::*;

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
pub fn App() -> impl IntoView {
    let categories_resource = create_resource(|| (), |_| async move { get_categories().await });
    let categories = move || {
        categories_resource
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
            .collect_view()
    };

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
