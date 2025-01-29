use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(pk_auto(PostTag::Id))
                    .col(integer(PostTag::PostId))
                    .col(integer(PostTag::TagId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    Id,
    PostId,
    TagId,
}
