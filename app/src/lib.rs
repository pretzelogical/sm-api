use crate::conf::db_conf;
use crate::routes::auth::{check_auth, log_in, register};
use crate::routes::post::{create_post, get_post};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use sea_orm::DatabaseConnection;
use sm_migration::{Migrator, MigratorTrait};

mod conf;
mod error;
mod middleware;
mod routes;
mod services;

pub const FILE_UPLOAD_ROOT: &str = "./media";

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_client: DatabaseConnection,
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let db_client = db_conf().await.unwrap();
    // Apply migrations
    Migrator::up(&db_client, None).await.unwrap();
    // Logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = AppState { db_client };
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::resource("/auth").route(web::get().to(check_auth)))
            .service(web::resource("/auth/login").route(web::post().to(log_in)))
            .service(web::resource("/auth/register").route(web::post().to(register)))
            // .service(
            //     web::resource("/user")
            //         .route(web::get().to(get_user))
            //         .wrap(middleware::auth::JWTSession),
            // )
            .service(
                web::resource("/post")
                    .route(web::get().to(get_post))
                    .route(web::post().to(create_post))
                    .wrap(middleware::auth::JWTSession),
            )
            // .service(
            //     web::resource("/post/{id}/comment")
            //         .route(web::get().to(get_comments))
            //         .route(web::post().to(create_comment))
            //         .wrap(middleware::auth::JWTSession),
            // )
            .service(Files::new(
                "/media/post",
                format!("{FILE_UPLOAD_ROOT}/post"),
            ))
            .wrap(cors)
            .wrap(Logger::default())
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
