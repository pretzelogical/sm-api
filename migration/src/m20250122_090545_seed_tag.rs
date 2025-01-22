use sea_orm::{Set, ActiveModelTrait, ModelTrait, EntityTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        sm_entity::tag::ActiveModel {
            id: Set(1),
            post_id: Set(1),
            name: Set("Test".to_owned())
        }
        .insert(db)
        .await
        .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let tag = sm_entity::tag::Entity::find_by_id(1)
            .one(db)
            .await?;
        if let Some(user) = tag {
            user.delete(db).await?;
        } else {
            panic!("Cannot find seeded tag");
        }
        Ok(())
    }
}

