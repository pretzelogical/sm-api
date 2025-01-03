use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use models::User;

mod models;

#[allow(unused_variables)]
#[get("/user/{user_id}")]
async fn get_user(path: web::Path<u64>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(User::default())
}

#[allow(unused_variables, unreachable_code)]
#[post("/user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    let user_data: User = user.into_inner();

    let client = todo!("implement deadpool_postgresql");

    HttpResponse::Ok().json(User::default())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
        .service(create_user)
        .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    server.await
}
