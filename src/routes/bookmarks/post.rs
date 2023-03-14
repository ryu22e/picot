use actix_web::{http::header::ContentType, HttpResponse};

pub async fn create_bookmark() -> HttpResponse {
    HttpResponse::Ok().body("test")
}
