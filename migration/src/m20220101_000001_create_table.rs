use sea_orm::Schema;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
#[allow(unused_variables, unreachable_code)]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(
                schema.create_table_from_entity(sm_entity::user::Entity)
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(sm_entity::user::Entity).to_owned())
            .await
    }
}

