use actix_web::{web, App, HttpServer};
use picot::routes::home;
use picot::schema::setup_schema;
use sea_orm::Database;
use std::io::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("sqlite::memory:")
        .await
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;
    setup_schema(&db)
        .await
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;
    HttpServer::new(|| App::new().route("/", web::get().to(home)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
