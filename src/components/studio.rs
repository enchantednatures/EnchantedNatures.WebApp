use leptos::prelude::*;
use leptos::suspense::Transition;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::get_photo;

#[derive(Params, PartialEq)]
struct StudioParams {
    id: usize,
}

#[component]
pub fn StudioPage() -> impl IntoView {
    let params = use_params::<StudioParams>();
    let id = move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or(1));
    let photo = LocalResource::new(move || async move { get_photo(id()).await });

    view! {
        <Transition
            fallback=move || {
                view! {
                    <p>
                        <em>"Loading..."</em>
                    </p>
                }
            }
        >
            {move || match photo.get() {
                None => {
                    view! {
                        <p>
                            <em>Error</em>
                        </p>
                    }
                        .into_any()
                }
                Some(photo) => {
                    view! {
                        <div class="gallery-body" id="studio-piece">
                            <img
                                alt=photo.title
                                class="gallery-image"
                                src=photo.cloudflare_resource
                            />
                            <div class="offset-md-3 offset-lg-2">
                                <p>{ photo.location_taken }</p>
                            </div>
                        </div>
                    }
                        .into_any()
                }
            }}

        </Transition>
    }
}
