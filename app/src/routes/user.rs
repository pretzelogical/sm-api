use crate::routes::prelude::*;
use crate::services::user::{get_by_id, get_by_name};


#[derive(Deserialize)]
pub struct GetUserArgs {
    pub id: Option<i64>,
    pub name: Option<String>,
}


pub async fn get_user(app_state: web::Data<AppState>, args: web::Query<GetUserArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    match (args.id, &args.name) {
        (Some(user_id), None) => {
            let user = get_by_id(user_id, db_client).await;
            match user {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(error) => error.into()
            }
        },
        (None, Some(user_name)) => match get_by_name(user_name, db_client).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => err.into()
        },
        (Some(_), Some(_)) => AppError::BadRequest(
            "Must have either 'id' or 'name' query params but not both").into(),
        (None, None) => AppError::BadRequest(
            "Must have 'id' or 'name' query params"
        ).into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserArgs {
    pub name: Option<String>,
    pub pass: Option<String>
}


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
                Err(err) => AppError::DbError(
                    AppDbError::from(err)
                ).into()
            }
        },
        (None, _) | (_, None) => {
            AppError::BadRequest("'name' or 'pass' field missing").into()
        }
    }
}


// #[delete("/user")]
// pub async fn create_user(app_state: web::Data<AppState>, user_args: web::Json<CreateUserArgs>) -> impl Responder {

// }
