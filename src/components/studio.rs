use leptos::*;
use leptos_router::*;

use crate::get_photo;

#[derive(Params, PartialEq)]
struct StudioParams {
    id: usize,
}

#[component]
pub fn StudioPage() -> impl IntoView {
    let params = use_params::<StudioParams>();
    let id = move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or(1));
    let (_pending, set_pending) = create_signal(false);
    let photo = create_resource(id, |value| async move { get_photo(value).await });

    view! {
        <Transition
            fallback=move || {
                view! {
                    <p>
                        <em>"Loading..."</em>
                    </p>
                }
            }

            set_pending
        >
            {move || match photo.get() {
                None => {
                    view! {
                        <p>
                            <em>Error</em>
                        </p>
                    }
                        .into_view()
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
                                <p>{{ photo.location_taken }}</p>
                            </div>
                        </div>
                    }
                        .into_view()
                }
            }}

        </Transition>
    }
}
