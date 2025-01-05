use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;
use crate::routes::{get_user, create_user};
use crate::conf::pg_conf;

mod conf;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = pg_conf();
    let pool = config.create_pool(None, NoTls).unwrap();
    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
            .service(create_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    server.await
}
