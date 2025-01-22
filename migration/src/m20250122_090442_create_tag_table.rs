use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    PostId,
    Name,
}

#[async_trait::async_trait]
#[allow(unused_variables, unreachable_code)]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id))
                    .col(integer(Tag::PostId))
                    .col(string(Tag::Name).unique_key())
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(sm_entity::tag::Entity).to_owned())
            .await
    }
}
