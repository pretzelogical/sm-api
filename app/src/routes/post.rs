use crate::{error::JsonError, routes::prelude::*};



#[derive(Deserialize)]
pub struct GetPostArgs {
  pub id: Option<i64>,
//   pub author: Option<String>
}

async fn get_by_id(post_id: i64, db_client: &DatabaseConnection) -> Result<post::Model, AppError> {
    let db_res = post::Entity::find_by_id(post_id)
        .one(db_client)
        .await;
    match db_res {
        Ok(post) => {
            match post {
                Some(post) => Ok(post),
                None => Err(AppError::NotFound)
            }
        },
        Err(err) => Err(AppError::DbError(err))
    }
}

#[get("/post")]
pub async fn get_post(app_state: web::Data<AppState>, args: web::Query<GetPostArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    match args.id {
        Some(post_id) => {
            let post = get_by_id(post_id, db_client).await;
            match post {
                Ok(post) => HttpResponse::Ok().json(post),
                Err(AppError::NotFound) => HttpResponse::NotFound()
                    .json(JsonError::from("post not found")),
                Err(AppError::DbError(err)) => HttpResponse::InternalServerError()
                    .json(JsonError::from(err)),
                _ => HttpResponse::InternalServerError().finish()
            }
        },
        None => HttpResponse::BadRequest()
            .json(JsonError::from("missing 'id' field"))
    }
}