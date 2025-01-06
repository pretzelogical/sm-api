pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250105_080123_seed_user;
mod m20250105_223608_create_post_table;
mod m20250105_232528_seed_post;
mod m20250106_071837_create_comment_table;
mod m20250106_072207_seed_comment;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250105_080123_seed_user::Migration),
            Box::new(m20250105_223608_create_post_table::Migration),
            Box::new(m20250105_232528_seed_post::Migration),
            Box::new(m20250106_071837_create_comment_table::Migration),
            Box::new(m20250106_072207_seed_comment::Migration),
        ]
    }
}
