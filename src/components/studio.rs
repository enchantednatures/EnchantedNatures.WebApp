use chrono::prelude::*;
use itertools::Itertools;
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
        <Transition fallback=move || {
            view! {
                <p>
                    <em>"Loading..."</em>
                </p>
            }
        }>

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
                    let description = vec![
                        &photo.title,
                        &photo.location_taken,
                        &photo.date_taken.format("%e %B %Y").to_string(),
                    ]
                        .iter()
                        .filter(|x| !x.is_empty())
                        .join(" // ")
                        .to_lowercase();
                    view! {
                        <div class="studio-body">
                            <img
                                alt=photo.title
                                class="studio-image"
                                src=photo.cloudflare_resource
                            />
                            <div class="studio-description">
                                <p>{description}</p>
                            </div>
                        </div>
                    }
                        .into_any()
                }
            }}

        </Transition>
    }
}

