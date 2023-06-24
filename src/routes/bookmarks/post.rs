use crate::app_state::AppState;
use crate::entities::bookmark;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Bookmark {
    title: String,
    url: String,
    // #[serde(default = "String::new")]
    // description: String,
    // #[serde(default)]
    // tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: i32,
    pub title: String,
    pub url: String,
    // pub description: String,
    // pub tags: Vec<String>,
}

pub async fn create_bookmark(
    data: web::Data<AppState>,
    form: web::Json<Bookmark>,
) -> Result<impl Responder> {
    let conn = &data.conn;
    let model =
        bookmark::create_bookmark(form.title.to_owned(), form.url.to_owned(), conn.to_owned())
            .await
            .expect("Failed to create bookmark");
    let obj = Response {
        id: model.id.unwrap(),
        title: model.title.unwrap(),
        url: model.url.unwrap(),
        // description: "".to_string(),
        // tags: form.tags.clone(),
    };
    Ok(web::Json(obj))
}
