use crate::app_state::AppState;
use crate::entities::bookmark;
use actix_web::{web, Responder, Result};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Bookmark {
    title: String,
    url: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub async fn create_bookmark(
    data: web::Data<AppState>,
    form: web::Json<Bookmark>,
) -> Result<impl Responder> {
    let conn = &data.conn;
    let b = bookmark::ActiveModel {
        title: Set(form.title.to_owned()),
        url: Set(form.url.to_owned()),
        ..Default::default()
    };
    // b.save(conn).await?;
    let obj = Response {
        id: 1,
        title: form.title.clone(),
        url: form.url.clone(),
        description: form.description.clone(),
        tags: form.tags.clone(),
    };
    Ok(web::Json(obj))
}
