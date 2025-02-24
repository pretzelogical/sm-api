use crate::services;
use crate::{routes::prelude::*, services::post::get_latest};
use actix_multipart::form::{bytes::Bytes as MpBytes, json::Json as MpJson, MultipartForm};

#[derive(Deserialize)]
pub struct GetPostArgs {
    pub id: Option<i64>,
    pub author_id: Option<i64>,
    pub limit: Option<u64>,
}

pub async fn get_post(
    app_state: web::Data<AppState>,
    args: web::Query<GetPostArgs>,
) -> Result<HttpResponse, AppError> {
    let db_client = &app_state.db_client;
    let limit = args.limit.unwrap_or(1);
    match (args.id, args.author_id, limit) {
        (None, None, _) => {
            let post = get_latest(db_client, 1).await?;
            Ok(HttpResponse::Ok().json(post))
        }
        (Some(_), Some(_), _) => Err(AppError::BadRequest(
            "Can provide either an 'id' or an 'author_id' field, but not both",
        )),
        (Some(post_id), None, _) => {
            let post = services::post::get_by_id(post_id, db_client).await?;
            Ok(HttpResponse::Ok().json(post))
        }
        (None, Some(author_id), limit) => {
            if limit > 0 {
                let post = services::post::get_by_author_id(author_id, limit, db_client).await?;
                Ok(HttpResponse::Ok().json(post))
            } else {
                Err(AppError::BadRequest(
                    "config.limit must be a number above 1",
                ))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePostArgs {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    // pub author_name: Option<String>,
    pub author_id: i64,
}

#[derive(Debug, MultipartForm)]
pub struct CreatePostForm {
    #[multipart(limit = "30MB")]
    pub img: Option<MpBytes>,
    pub post: MpJson<CreatePostArgs>,
}

pub async fn create_post(
    app_state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<CreatePostForm>,
) -> Result<HttpResponse, AppError> {
    let post = services::post::create_post(form, &app_state.db_client).await?;
    Ok(HttpResponse::Ok().json(post))
}

// #[derive(Deserialize)]
// pub struct GetCommentsArgs {
//     pub limit: Option<u64>,
// }
//
// pub async fn get_comments(
//     app_state: web::Data<AppState>,
//     post_id: web::Path<i64>,
//     args: web::Query<GetCommentsArgs>,
// ) -> impl Responder {
//     let post = sm_entity::post::Entity::find_by_id(post_id.into_inner())
//         .one(&app_state.db_client)
//         .await;
//     if let Ok(Some(post)) = post {
//         match services::post::get_comments(&post, &app_state.db_client, args.limit).await {
//             Ok(Some(comments)) => HttpResponse::Ok().json(comments),
//             _ => AppError::NotFound("comment not found").into(),
//         }
//     } else {
//         AppError::NotFound("comment not found").into()
//     }
// }

// #[derive(Deserialize)]
// pub struct CreateCommentForm {
//     pub author_id: i64,
//     pub title: String,
//     pub content: String,
// }
//
// #[derive(Deserialize)]
// pub struct CreateCommentArgs {
//     pub comment: CreateCommentForm,
// }
//
// pub async fn create_comment(
//     app_state: web::Data<AppState>,
//     post_id: web::Path<i64>,
//     args: web::Json<CreateCommentArgs>,
// ) -> impl Responder {
//     use ActiveValue::Set;
//     let comment_form = &args.comment;
//     let comment = sm_entity::comment::ActiveModel {
//         post_id: Set(post_id.into_inner()),
//         author_id: Set(comment_form.author_id),
//         title: Set(comment_form.title.clone()),
//         content: Set(comment_form.content.clone()),
//         ..Default::default()
//     }
//     .insert(&app_state.db_client)
//     .await;
//
//     match comment {
//         Ok(comment) => HttpResponse::Ok().json(comment),
//         Err(error) => AppError::DbError(AppDbError::from(error)).into(),
//     }
// }
