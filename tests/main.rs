#[cfg(test)]
mod tests {
    use actix_web::{http, http::header::ContentType, test, web, App};

    use picot::routes::home;

    #[actix_web::test]
    async fn test_home_unit() {
        let resp = home().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_home_get() {
        let app = test::init_service(App::new().route("/", web::get().to(home))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
