use sea_orm_migration::prelude::*;
use sea_orm::{Set, ActiveModelTrait, ModelTrait, EntityTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        sm_entity::post::ActiveModel {
            id: Set(1),
            author_id: Set(1),
            title: Set("Test post 1".to_owned()),
            content: Set("Test post content :3:3:3".to_owned())
        }
        .insert(db)
        .await
        .unwrap();

        sm_entity::post::ActiveModel {
            id: Set(2),
            author_id: Set(1),
            title: Set("Test post 2".to_owned()),
            content: Set("ashdsajd".to_owned())
        }
        .insert(db)
        .await
        .unwrap();

        sm_entity::post::ActiveModel {
            id: Set(3),
            author_id: Set(1),
            title: Set("Test post 3".to_owned()),
            content: Set("funni".to_owned())
        }
        .insert(db)
        .await
        .unwrap();

        sm_entity::post::ActiveModel {
            id: Set(4),
            author_id: Set(1),
            title: Set("Test post 4".to_owned()),
            content: Set("haha".to_owned())
        }
        .insert(db)
        .await
        .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        for i in 1..=4 {
            let post = sm_entity::post::Entity::find_by_id(i)
                .one(db)
                .await?;
            if let Some(post) = post {
                post.delete(db).await?;
            } else {
                panic!("Cannot find seeded post");
            }
        }

        Ok(())
    }
}
