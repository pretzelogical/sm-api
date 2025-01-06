use crate::routes::prelude::*;
use sea_orm::{QueryFilter, QueryOrder, QuerySelect};

#[derive(Deserialize)]
pub struct GetPostArgs {
    pub id: Option<i64>,
    pub author_id: Option<i64>,
    pub limit: Option<u64>,
}

async fn get_by_id(post_id: i64, db_client: &DatabaseConnection) -> Result<post::Model, AppError> {
    let db_res = post::Entity::find_by_id(post_id).one(db_client).await;
    match db_res {
        Ok(post) => match post {
            Some(post) => Ok(post),
            None => Err(AppError::NotFound("post not found")),
        },
        Err(err) => Err(AppError::DbError(AppDbError::from(err))),
    }
}

async fn get_by_author_id(
    author_id: i64,
    limit: u64,
    db_client: &DatabaseConnection,
) -> Result<Vec<post::Model>, AppError> {
    let db_res = post::Entity::find()
        .filter(post::Column::AuthorId.eq(author_id))
        .order_by_asc(post::Column::AuthorId)
        .limit(limit)
        .all(db_client)
        .await;
    match db_res {
        Ok(posts) => Ok(posts),
        Err(err) => Err(AppError::DbError(AppDbError::from(err))),
    }
}

#[get("/post")]
pub async fn get_post(
    app_state: web::Data<AppState>,
    args: web::Query<GetPostArgs>,
) -> impl Responder {
    let db_client = &app_state.db_client;
    let limit = match args.limit {
        Some(limit) => limit,
        None => 1,
    };
    match (args.id, args.author_id, limit) {
        (None, None, _) => {
            AppError::BadRequest("Must provide an 'id' or an 'author_id' field").into()
        }
        (Some(_), Some(_), _) => AppError::BadRequest(
            "Must provide either an 'id' or an 'author_id' field, but not both",
        )
        .into(),
        (Some(post_id), None, _) => match get_by_id(post_id, db_client).await {
            Ok(post) => HttpResponse::Ok().json(post),
            Err(err) => err.into(),
        },
        (None, Some(author_id), limit) => {
            if limit > 0 {
                match get_by_author_id(author_id, limit, db_client).await {
                    Ok(post) => HttpResponse::Ok().json(post),
                    Err(err) => err.into(),
                }
            } else {
                AppError::BadRequest("config.limit must be a number above 1").into()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePostArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    // pub author_name: Option<String>,
    pub author_id: Option<i64>,
}

async fn _create_post(
    title: &String,
    content: &String,
    author_id: &i64,
    db_client: &DatabaseConnection,
) -> Result<post::Model, AppError> {
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
        Err(err) => Err(AppError::DbError(AppDbError::from(err))),
    }
}

#[post("/post")]
pub async fn create_post(
    app_state: web::Data<AppState>,
    args: web::Json<CreatePostArgs>,
) -> impl Responder {
    match (&args.title, &args.content, &args.author_id) {
        (Some(title), Some(content), Some(author_id)) => {
            match _create_post(title, content, author_id, &app_state.db_client).await {
                Ok(post) => HttpResponse::Ok().json(post),
                Err(err) => err.into(),
            }
        },
        _ => AppError::InternalError("error creating user").into(),
    }
}
