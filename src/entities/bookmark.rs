use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "bookmarks")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub url: String,
    // TODO Tagを追加する
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create_bookmark(
    title: String,
    url: String,
    conn: DatabaseConnection,
) -> Result<ActiveModel, DbErr> {
    let model = ActiveModel {
        title: Set(title.to_owned()),
        url: Set(url.to_owned()),
        ..Default::default()
    }
    .save(&conn)
    .await?;
    Ok(model)
}
