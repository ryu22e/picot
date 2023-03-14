use actix_web::{web, App, HttpServer};
use picot::routes::home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO bookmark追加
    HttpServer::new(|| App::new().route("/", web::get().to(home)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
