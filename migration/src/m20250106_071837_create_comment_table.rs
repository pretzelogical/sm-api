use sea_orm_migration::prelude::*;
use sea_orm::Schema;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(
                schema.create_table_from_entity(sm_entity::comment::Entity)
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(sm_entity::comment::Entity).to_owned())
            .await
    }
}
