use actix_web::{web, App, HttpServer};
use picot::routes::home;
use picot::schema::setup_schema;
use sea_orm::Database;
use std::io::Error;

const SQLITE_FILE_PATH: &str = "/Users/ryu22e/development/picot/db.sqlite";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !std::path::Path::new(SQLITE_FILE_PATH).exists() {
        std::fs::File::create(SQLITE_FILE_PATH)?;
    }
    // TODO ファイルパスを指定する
    let db = Database::connect(format!("sqlite://{}", SQLITE_FILE_PATH))
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
