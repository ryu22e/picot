pub use sea_orm_migration::prelude::*;

mod m20231125_065406_create_table_bookmarks;
mod m20231125_065410_create_table_tags;
mod m20231125_073536_create_table_bookmarks_tags;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231125_065406_create_table_bookmarks::Migration),
            Box::new(m20231125_065410_create_table_tags::Migration),
            Box::new(m20231125_073536_create_table_bookmarks_tags::Migration),
        ]
    }
}
