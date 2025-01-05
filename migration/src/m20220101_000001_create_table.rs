use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
#[allow(unused_variables, unreachable_code)]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(SmUser::Table)
                    .if_not_exists()
                    .col(pk_auto(SmUser::Id))
                    .col(string(SmUser::Name))
                    .col(string(SmUser::Pass))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(SmUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SmUser {
    Table,
    Id,
    Name,
    Pass,
}
