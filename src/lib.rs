use leptos::*;
use leptos_router::*;
mod api;
pub mod components;
pub mod directives;
use api::*;

use self::components::gallery::GalleryPage;
use self::components::gallery::Portfolio;
use self::components::studio::StudioPage;

#[component]
pub fn App() -> impl IntoView {
    let categories_resource = create_resource(|| (), |_| async move { get_categories().await });
    let categories = {
        move || match categories_resource.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => data
                .into_iter()
                .map(|category| {
                    view! {
                        <li class="controller-link">
                            <A href=format!( "/gallery/{}", category.id,)>{category.name}</A>
                        </li>
                    }
                })
                .collect_view(),
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
                        <Routes>
                            <Route path="/" view=Portfolio/>
                            <Route path="/gallery/:id" view=GalleryPage/>
                            <Route path="/studio/:id" view=StudioPage/>
                        </Routes>
                    </main>
                </Router>
            </div>
        </div>
    }
}
