use crate::routes::prelude::*;


#[derive(Deserialize)]
pub struct GetUserArgs {
    pub id: Option<i64>,
    // pub name: String,
}

async fn get_by_id(user_id: i64, db_client: &DatabaseConnection) -> Result<user::Model, AppError> {
    let db_res = user::Entity::find_by_id(user_id)
        .one(db_client)
        .await;
    match db_res {
        Ok(user) => {
            match user {
                Some(user) => Ok(user),
                None => Err(AppError::NotFound(Some("user not found")))
            }
        }
        Err(err) => Err(AppError::DbError(AppDbError::from(err)))
    }
}


#[get("/user")]
pub async fn get_user(app_state: web::Data<AppState>, args: web::Query<GetUserArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    match args.id {
        Some(user_id) => {
            let user = get_by_id(user_id, db_client).await;
            match user {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(error) => error.into()
            }
        },
        None => AppError::BadRequest(Some("Missing 'id' field")).into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserArgs {
    pub name: Option<String>,
    pub pass: Option<String>
}

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
                Err(err) => AppError::DbError(
                    AppDbError::from(err)
                ).into()
            }
        },
        (None, _) | (_, None) => {
            AppError::BadRequest(Some("'name' or 'pass' field missing")).into()
        }
    }
}


// #[delete("/user")]
// pub async fn create_user(app_state: web::Data<AppState>, user_args: web::Json<CreateUserArgs>) -> impl Responder {

// }
