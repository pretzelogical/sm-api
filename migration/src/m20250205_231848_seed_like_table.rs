use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        sm_entity::like::ActiveModel {
            id: Set(1),
            user_id: Set(1),
            post_id: Set(1),
        }
        .insert(db)
        .await
        .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let like = sm_entity::like::Entity::find_by_id(1).one(db).await?;
        if let Some(like) = like {
            like.delete(db).await?;
        } else {
            panic!("Cannot find seeded like");
        }
        Ok(())
    }
}
