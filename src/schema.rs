use crate::entities::bookmark::Entity as Bookmark;
use sea_orm::*;

pub async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let st = builder.build(&schema.create_table_from_entity(Bookmark));
    db.execute(st).await?;
    Ok(())
}
