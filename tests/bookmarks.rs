#[cfg(test)]
mod tests {
    use actix_web::{http, http::header::ContentType, test, web, App};
    use picot::app_state::AppState;
    use picot::routes::create_bookmark;
    use picot::routes::Response;
    use picot::schema::setup_schema;
    use sea_orm::Database;
    use serde_json::json;

    #[actix_web::test]
    async fn create_bookmark_unit() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        setup_schema(&db).await.unwrap();
        let state = AppState { conn: db };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .route("/", web::post().to(create_bookmark)),
        )
        .await;
        let payload = json!({
            "title": "test",
            "url": "https://example.com",
            "description": "test data",
            "tags": ["test1", "test2"],
        });
        let req = test::TestRequest::post()
            .uri("/")
            .set_payload(payload.to_string())
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            http::StatusCode::OK,
            "{:?}",
            resp.response().body()
        );
        let actual: Response = test::read_body_json(resp).await;
        assert_eq!(actual.title, "test");
        assert_eq!(actual.url, "https://example.com");
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
