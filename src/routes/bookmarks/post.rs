use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Bookmark {
    title: String,
    url: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub async fn create_bookmark(bookmark: web::Json<Bookmark>) -> Result<impl Responder> {
    let obj = Response {
        id: 1,
        title: bookmark.title.clone(),
        url: bookmark.url.clone(),
        description: bookmark.description.clone(),
        tags: bookmark.tags.clone(),
    };
    Ok(web::Json(obj))
}
