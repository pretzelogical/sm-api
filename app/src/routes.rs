use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use crate::AppState;
use serde_json::json;
use sm_entity::user;


#[derive(Deserialize)]
pub struct GetUserArgs {
    pub id: Option<i64>,
    // pub name: String,
}

#[allow(unused_variables)]
#[get("/user")]
pub async fn get_user(app_state: web::Data<AppState>, args: web::Query<GetUserArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    let user_id = args.id;
    match user_id {
        Some(user_id) => {
            let user = user::Entity::find_by_id(user_id)
                .one(db_client)
                .await
                .expect("Could not find user");
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
pub async fn create_user(app_state: web::Data<AppState>, user_args: web::Json<CreateUserArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    let name = &user_args.name;
    let pass = &user_args.pass;
    match (name, pass) {
        (Some(name), Some(pass)) => {
            let new_user = user::ActiveModel {
                name: ActiveValue::Set(name.to_owned()),
                pass: ActiveValue::Set(pass.to_owned()),
                ..Default::default()
            }
            .insert(db_client)
            .await;
            match new_user {
                Ok(new_user) => HttpResponse::Ok().json(new_user),
                Err(error) => HttpResponse::InternalServerError()
                    .json(json!({ "error": "Could not create new user"}))
            }
        },
        (None, _) | (_, None) => {
            HttpResponse::BadRequest()
                .json(json!({ "error": "'name' or 'pass' field missing" }))
        }
    }
}