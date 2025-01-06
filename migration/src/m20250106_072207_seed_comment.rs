use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        for post_id in 1..=4 {
            for idx in 1..=4 {
                sm_entity::comment::ActiveModel {
                    author_id: Set(1),
                    post_id: Set(post_id),
                    title: Set(format!("Test comment {idx}")),
                    content: Set("Test comment content :3:3:3".to_owned()),
                    ..Default::default()
                }
                .insert(db)
                .await
                .unwrap();
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        for comment in sm_entity::comment::Entity::find()
            .filter(sm_entity::comment::Column::AuthorId.eq(1))
            .all(db)
            .await?
        {
            comment.delete(db).await?;
        }

        Ok(())
    }
}
