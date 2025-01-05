use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use sm_migration::{Migrator, MigratorTrait};
use crate::routes::{get_user, create_user};
use crate::conf::db_conf;

mod conf;
mod routes;

#[derive(Debug, Clone)]
struct AppState {
    db_client: DatabaseConnection
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let db_client = db_conf().await.unwrap();
    // Apply migrations
    Migrator::up(&db_client, None).await.unwrap();
    let app_state = AppState { db_client };
    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone()))
            .service(create_user)
            .service(get_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run();

    server.await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}