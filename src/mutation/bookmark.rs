use crate::entities::{bookmark, bookmark::ActiveModel as Bookmark};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;

pub struct Mutation;

impl Mutation {
    pub async fn create_bookmark(
        conn: &DatabaseConnection,
        form_data: bookmark::Model,
    ) -> Result<Bookmark, DbErr> {
        let model = Bookmark {
            title: Set(form_data.title.to_owned()),
            url: Set(form_data.url.to_owned()),
            description: Set(form_data.description.to_owned()),
            ..Default::default()
        }
        .save(conn)
        .await?;
        Ok(model)
    }
}
