use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bookmark {
    title: String,
    url: String,
    // description: String,
    // tags: Vec<String>,
}

pub async fn create_bookmark(bookmark: web::Json<Bookmark>) -> HttpResponse {
    HttpResponse::Ok().body("test")
}
