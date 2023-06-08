use crate::app_state::AppState;
use crate::entities::bookmark;
use actix_web::{web, Responder, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::Database;
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
        title: Set(form.title.clone()),
        url: Set(form.url.clone()),
        ..Default::default()
    };
    //bookmark::Entity::insert(b).exec(conn).await?;
    let obj = Response {
        id: 1,
        title: form.title.clone(),
        url: form.url.clone(),
        description: form.description.clone(),
        tags: form.tags.clone(),
    };
    Ok(web::Json(obj))
}
