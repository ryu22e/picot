use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Bookmark {
    title: String,
    url: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(default)]
    tags: Vec<String>,
}

pub async fn create_bookmark(bookmark: web::Json<Bookmark>) -> Result<impl Responder> {
    let obj = Bookmark {
        title: bookmark.title.clone(),
        url: bookmark.url.clone(),
        description: bookmark.description.clone(),
        tags: bookmark.tags.clone(),
    };
    Ok(web::Json(obj))
}
