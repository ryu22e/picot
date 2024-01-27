use crate::app_state::AppState;
use crate::entities::bookmark;
use crate::mutation::bookmark::Mutation;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub async fn create_bookmark(
    data: web::Data<AppState>,
    post_form: web::Json<bookmark::Model>,
) -> Result<impl Responder> {
    let conn = &data.conn;
    let form = post_form.into_inner();
    let model = Mutation::create_bookmark(conn, form)
        .await
        .expect("could not insert bookmark");
    let obj = Response {
        id: model.id.unwrap(),
        title: model.title.unwrap(),
        url: model.url.unwrap(),
        // tags: model.tags.unwrap(),
        // TODO DBから取得する
        tags: vec!["test1".to_string(), "test2".to_string()],
        description: model.description.unwrap(),
    };
    Ok(web::Json(obj))
}
