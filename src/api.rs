use reqwest::Client;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct CategoryViewModel {
    pub id: usize,
    pub name: String,
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

pub async fn get_photos(category_id: usize) -> Vec<PhotoViewModel> {
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

pub async fn get_categories() -> Vec<CategoryViewModel> {
    let url = "https://enchantednatures.shuttleapp.rs/api/v0/categories";
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let categories: Vec<CategoryViewModel> = resp.json().await.unwrap();
    dbg!(&categories);
    categories
}

