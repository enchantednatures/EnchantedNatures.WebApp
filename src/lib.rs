use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;
mod api;
pub mod components;
pub mod directives;
use api::*;

use self::components::gallery::GalleryPage;
use self::components::gallery::Portfolio;
use self::components::studio::StudioPage;

#[component]
pub fn App() -> impl IntoView {
    let categories_resource = LocalResource::new(|| async move { get_categories().await });
    let categories = {
        move || match categories_resource.get() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(data) => data
                .into_iter()
                .map(|category| {
                    view! {
                        <li class="controller-link">
                            <A href=format!( "/gallery/{}", category.id,)>{category.name}</A>
                        </li>
                    }
                })
                .collect_view().into_any(),
        }
    };

    view! {
        <div class="container-fluid">
            <div class="row">
                <Router>
                    <nav class="col-md-3 col-lg-2 bg-white sidebar">
                        <div id="sidebar-nav" class="sidebar-sticky pt-3">
                            <ul class="nav flex-column">
                                <li class="controller-link">
                                    <A href="/">enchanted natures</A>
                                </li>
                                {categories}
                            </ul>
                        </div>
                    </nav>
                    <main>
                        <Routes fallback=|| view! { <p>"Not Found"</p> }>
                            <Route path=path!("/") view=Portfolio/>
                            <Route path=path!("/gallery/:id") view=GalleryPage/>
                            <Route path=path!("/studio/:id") view=StudioPage/>
                        </Routes>
                    </main>
                </Router>
            </div>
        </div>
    }
}
