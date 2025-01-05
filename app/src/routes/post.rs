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
                _ => HttpResponse::InternalServerError()
                    .json(JsonError::from("could not find post"))
            }
        },
        None => HttpResponse::BadRequest()
            .json(JsonError::from("missing 'id' field"))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePostArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    // pub author_name: Option<String>,
    pub author_id: Option<i64>
}

async fn _create_post(
    title: &String,
    content: &String,
    author_id: &i64,
    db_client: &DatabaseConnection) -> Result<post::Model, AppError> {
    let new_post = post::ActiveModel {
        title: ActiveValue::Set(title.to_owned()),
        content: ActiveValue::Set(content.to_owned()),
        author_id: ActiveValue::Set(author_id.to_owned()),
        ..Default::default()
    }
        .insert(db_client)
        .await;
    match new_post {
        Ok(post) => Ok(post),
        Err(err) => Err(AppError::DbError(err))
    }
}

#[post("/post")]
pub async fn create_post(app_state: web::Data<AppState>, args: web::Json<CreatePostArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    let title = &args.title;
    let content = &args.content;
    let author_id = &args.author_id;
    let default_error = "error creating user";
    match (title, content, author_id) {
        (Some(title), Some(content), Some(author_id)) => {
            match _create_post(title, content, author_id, db_client).await {
                Ok(post) =>
                    HttpResponse::Ok().json(post),
                Err(AppError::DbError(err)) =>
                    HttpResponse::InternalServerError()
                        .json(JsonError::from(err)),
                _ => HttpResponse::InternalServerError()
                    .json(JsonError::from(default_error))
            }
        },
        _ => HttpResponse::InternalServerError()
                .json(JsonError::from(default_error))
    }
}