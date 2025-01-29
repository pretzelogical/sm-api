use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        sm_entity::post_tag::ActiveModel {
            id: Set(1),
            post_id: Set(1),
            tag_id: Set(1),
        }
        .insert(db)
        .await
        .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let post_tag = sm_entity::post_tag::Entity::find_by_id(1).one(db).await?;
        if let Some(post_tag) = post_tag {
            post_tag.delete(db).await?;
        } else {
            panic!("Cannot find seeded post_tag");
        }
        Ok(())
    }
}
