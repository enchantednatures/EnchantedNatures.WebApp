use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;

use crate::directives::horizontal_scroll;
use crate::get_photos;
use crate::PhotoViewModel;
use crate::PhotoViewModel2;

#[derive(Params, PartialEq)]
struct GalleryParams {
    id: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GalleryViewModel {
    pub photos: Vec<PhotoViewModel>,
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let photos = create_resource(|| (), |_| async move { get_photos(1).await });

    let photo_view = {
        move || {
            match photos.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => data
                    .photos
                    .into_iter()
                    .map(|photo| {
                        view! { <GalleryPiece photo/> }
                    })
                    .collect_view(),
            }
            .into_view()
        }
    };

    view! { <Gallery>{photo_view}</Gallery> }
}

#[component]
pub fn GalleryPage() -> impl IntoView {
    let params = use_params::<GalleryParams>();
    let id = move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or(1));
    let photos = create_resource(id, |value| async move { get_photos(value).await });

    let photo_view = {
        move || {
            match photos.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => data
                    .photos
                    .into_iter()
                    .map(|photo| {
                        view! { <GalleryPiece photo/> }
                    })
                    .collect_view(),
            }
            .into_view()
        }
    };

    view! { <Gallery>{photo_view}</Gallery> }
}

#[component]
pub fn GalleryPiece(photo: PhotoViewModel2) -> impl IntoView {
    view! {
        <a href=format!("/studio/{}", photo.id)>
            <img class="gallery-image" src=photo.cloudflare_resource alt=photo.title/>
        </a>
    }
}

#[component]
pub fn Gallery(children: Children) -> impl IntoView {
    view! {
        <div class="gallery-body" id="gallery-piece" use:horizontal_scroll>
            {children()}
        </div>
    }
}
