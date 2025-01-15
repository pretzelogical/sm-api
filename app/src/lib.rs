use actix_web::dev::HttpServiceFactory;
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use sm_migration::{Migrator, MigratorTrait};
use crate::routes::user::{get_user, create_user};
use crate::routes::post::{get_post, create_post, get_comments, create_comment};
use crate::conf::db_conf;

mod conf;
mod routes;
mod error;
mod services;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_client: DatabaseConnection
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let db_client = db_conf().await.unwrap();
    // Apply migrations
    Migrator::up(&db_client, None).await.unwrap();
    let app_state = AppState { db_client };
    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone()))
            .service(
                web::resource("/user")
                    .route(web::get().to(get_user))
                    .route(web::post().to(create_user))
            )
            .service(
                web::resource("/post")
                    .route(web::get().to(get_post))
                    .route(web::post().to(create_post))
            )
            .service(
                web::resource("/post/{id}/comment")
                    .route(web::get().to(get_comments))
                    .route(web::post().to(create_comment))
            )
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