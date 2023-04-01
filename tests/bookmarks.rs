#[cfg(test)]
mod tests {
    use actix_web::{http, http::header::ContentType, test, web, App};

    use picot::routes::create_bookmark;
    use picot::routes::Bookmark;
    use serde_json::json;

    #[actix_web::test]
    async fn create_bookmark_unit() {
        let app = test::init_service(App::new().route("/", web::post().to(create_bookmark))).await;
        let payload = json!({
            "title": "test",
            "url": "https://example.com",
        });
        let req = test::TestRequest::post()
            .uri("/")
            .set_payload(payload.to_string())
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let bookmark: Bookmark = test::read_body_json(resp).await;
    }

    // #[actix_web::test]
    // async fn test_home_get() {
    //     let app = test::init_service(App::new().route("/", web::get().to(home))).await;
    //     let req = test::TestRequest::default()
    //         .insert_header(ContentType::plaintext())
    //         .to_request();
    //     let resp = test::call_service(&app, req).await;
    //     assert!(resp.status().is_success());
    // }
}
