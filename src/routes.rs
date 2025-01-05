use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};

use crate::models::User;



#[derive(Deserialize)]
pub struct GetUserArgs {
    pub id: Option<i64>,
    // pub name: String,
}

#[allow(unused_variables)]
#[get("/user")]
pub async fn get_user(args: web::Query<GetUserArgs>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.unwrap();
    let user_id = args.id;
    match user_id {
        Some(user_id) => {
            let user = User::get(&client, user_id).await;
            match user {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().body("Error: user not found")
            }
        },
        None => HttpResponse::BadRequest().body("Error: no id in query args")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserArgs {
    pub name: Option<String>,
    pub pass: Option<String>
}

#[allow(unused_variables, unreachable_code)]
#[post("/user")]
pub async fn create_user(user_args: web::Json<CreateUserArgs>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.unwrap();
    let user_args = user_args.into_inner();
    if user_args.name == None || user_args.pass == None {
        return HttpResponse::BadRequest().body("Error: 'name' or 'pass' field missing");
    }
    let new_user = User::create(&client, user_args).await;
    match new_user {
        Some(new_user) => HttpResponse::Ok().json(new_user),
        None => HttpResponse::InternalServerError().body("Error: could not create user")
    }
}