use std::cell::RefCell;
use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

const R2_BASE: &str = "https://r2.enchantednatures.com";

struct AppData {
    resources: BTreeMap<usize, String>,
    categories: BTreeMap<usize, String>,
    photos: BTreeMap<usize, GalleryViewModel>,
}

thread_local! {
    static CACHE: RefCell<Option<AppData>> = RefCell::new(None);
}

pub async fn init_data() {
    let already_init = CACHE.with(|c| c.borrow().is_some());
    if already_init {
        return;
    }

    log::debug!("fetching app data from r2...");

    let resources = match fetch_json::<BTreeMap<usize, String>>(
        &format!("{}/cloudflare_resources.json", R2_BASE),
    )
    .await
    {
        Ok(v) => {
            log::debug!("loaded {} cloudflare resources", v.len());
            v
        }
        Err(e) => {
            log::error!("cloudflare_resources fetch failed: {}", e);
            BTreeMap::new()
        }
    };

    let categories = match fetch_json::<BTreeMap<usize, String>>(
        &format!("{}/categories.json", R2_BASE),
    )
    .await
    {
        Ok(v) => {
            log::debug!("loaded {} categories", v.len());
            v
        }
        Err(e) => {
            log::error!("categories fetch failed: {}", e);
            BTreeMap::new()
        }
    };

    let photos = match fetch_json::<BTreeMap<usize, GalleryViewModel>>(
        &format!("{}/photos_by_category.json", R2_BASE),
    )
    .await
    {
        Ok(v) => {
            log::debug!("loaded {} photo categories", v.len());
            v
        }
        Err(e) => {
            log::error!("photos_by_category fetch failed: {}", e);
            BTreeMap::new()
        }
    };

    CACHE.with(|c| {
        if c.borrow().is_none() {
            *c.borrow_mut() = Some(AppData {
                resources,
                categories,
                photos,
            });
            log::debug!("app data cached");
        }
    });
}

async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    log::debug!("fetching {}", url);
    let resp = gloo_net::http::Request::get(url)
        .send()
        .await
        .map_err(|e| format!("request failed: {:?}", e))?;

    let status = resp.status();
    log::debug!("{} -> status {}", url, status);

    if !resp.ok() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, body));
    }

    resp.json::<T>()
        .await
        .map_err(|e| format!("json parse failed: {:?}", e))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct CategoryViewModel {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PhotoViewModel {
    pub id: usize,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PhotoViewModel2 {
    pub id: usize,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub cloudflare_resource: String,
}

impl From<PhotoViewModel> for PhotoViewModel2 {
    fn from(value: PhotoViewModel) -> Self {
        let resource_id = CACHE.with(|c| {
            c.borrow()
                .as_ref()
                .and_then(|data| data.resources.get(&value.id).cloned())
        });
        let resource_id = resource_id.unwrap_or_else(|| {
            log::warn!("missing cloudflare resource for photo {}", value.id);
            String::new()
        });
        PhotoViewModel2 {
            id: value.id,
            title: value.title,
            filename: value.filename,
            location_taken: value.location_taken,
            date_taken: value.date_taken,
            cloudflare_resource: format!(
                "https://imagedelivery.net/ubbJTaQO-oc8x-62MYWhKg/{}/public",
                resource_id
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GalleryViewModel {
    pub photos: Vec<PhotoViewModel>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GalleryViewModel2 {
    pub photos: Vec<PhotoViewModel2>,
}

impl From<GalleryViewModel> for GalleryViewModel2 {
    fn from(value: GalleryViewModel) -> Self {
        Self {
            photos: value.photos.into_iter().map(|v| v.into()).collect(),
        }
    }
}

pub async fn get_photos(category_id: usize) -> GalleryViewModel2 {
    init_data().await;
    let gallery = CACHE.with(|c| {
        c.borrow()
            .as_ref()
            .and_then(|data| data.photos.get(&category_id).map(|g| g.clone()))
            .unwrap_or_else(|| GalleryViewModel { photos: vec![] })
    });
    gallery.into()
}

pub async fn get_photo(photo_id: usize) -> PhotoViewModel2 {
    init_data().await;
    let photo = CACHE.with(|c| {
        c.borrow()
            .as_ref()
            .and_then(|data| {
                data.photos
                    .iter()
                    .flat_map(|(_, v)| v.photos.clone())
                    .find(|v| v.id == photo_id)
            })
    });
    match photo {
        Some(p) => p.into(),
        None => {
            log::error!("photo {} not found", photo_id);
            PhotoViewModel {
                id: photo_id,
                title: "Not Found".into(),
                filename: String::new(),
                location_taken: String::new(),
                date_taken: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            }
            .into()
        }
    }
}

pub async fn get_categories() -> Vec<CategoryViewModel> {
    init_data().await;
    let categories = CACHE.with(|c| {
        c.borrow()
            .as_ref()
            .map(|data| data.categories.clone())
            .unwrap_or_default()
    });
    categories
        .iter()
        .map(|(id, name)| CategoryViewModel {
            id: *id,
            name: name.clone(),
        })
        .collect()
}
