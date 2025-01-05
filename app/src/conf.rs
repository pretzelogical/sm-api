use sea_orm::{Database, DatabaseConnection};


pub async fn db_conf() -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect("sqlite::memory:")
        .await
}
