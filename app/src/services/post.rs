use sea_orm::{
  ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
  QueryFilter, QueryOrder, QuerySelect, ActiveValue, ActiveModelTrait
};
use serde::Serialize;
use sm_entity::post;

use crate::routes::post::CreatePostForm;
use crate::services::image::upload_post_img;
use crate::error::{AppDbError, AppError};


#[derive(Serialize)]
pub struct PostResponseItem {
    pub post: sm_entity::post::Model,
    pub comment: Option<Vec<sm_entity::comment::Model>>,
    pub tag: Option<Vec<sm_entity::tag::Model>>
}


pub async fn get_comments(
  post: &sm_entity::post::Model,
  db_client: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Option<Vec<sm_entity::comment::Model>>, AppError> {
  let comments = if let Some(limit) = limit {
      post.find_related(sm_entity::comment::Entity)
          .limit(limit)
          .all(db_client)
          .await
  } else {
      post.find_related(sm_entity::comment::Entity)
          .all(db_client)
          .await
  };
  match comments {
      Ok(comments) => {
          if comments.len() > 0 {
              Ok(Some(comments))
          } else {
              Ok(None)
          }
      }
      Err(error) => Err(AppError::DbError(AppDbError::from(error))),
  }
}

pub async fn get_tags(
    post: &sm_entity::post::Model,
    db_client: &DatabaseConnection
) -> Result<Option<Vec<sm_entity::tag::Model>>, AppError> {
    match post.find_related(sm_entity::tag::Entity)
        .all(db_client)
        .await {
            Ok(tags) => {
                if tags.len() > 0 {
                    Ok(Some(tags))
                } else {
                    Ok(None)
                }
            },
            Err(err) => Err(AppError::DbError(AppDbError::from(err)))
        }
}

pub async fn get_by_id(
  post_id: i64,
  db_client: &DatabaseConnection,
) -> Result<PostResponseItem, AppError> {
  let db_res = post::Entity::find_by_id(post_id).one(db_client).await;
  match db_res {
      Ok(post) => match post {
          Some(post) => Ok(PostResponseItem {
              comment: get_comments(&post, db_client, None).await?,
              tag: get_tags(&post, db_client).await?,
              post: post,
          }),
          None => Err(AppError::NotFound("post not found")),
      },
      Err(err) => Err(AppError::DbError(AppDbError::from(err))),
  }
}

pub async fn get_by_author_id(
  author_id: i64,
  limit: u64,
  db_client: &DatabaseConnection,
) -> Result<Vec<PostResponseItem>, AppError> {
  let db_res = post::Entity::find()
      .filter(post::Column::AuthorId.eq(author_id))
      .order_by_asc(post::Column::AuthorId)
      .limit(limit)
      .all(db_client)
      .await;
  match db_res {
      Ok(posts) => {
          let mut item_vec = Vec::with_capacity(posts.len());
          for post in posts {
              item_vec.push(
                PostResponseItem {
                    comment: get_comments(&post, db_client, None).await?,
                    tag: get_tags(&post, db_client).await?,
                    post: post
                }
              );
          }
          Ok(item_vec)
      }
      Err(err) => Err(AppError::DbError(AppDbError::from(err))),
  }
}

pub async fn create_post(
    form: CreatePostForm,
    db_client: &DatabaseConnection,
) -> Result<post::Model, AppError> {
    let img_path = upload_post_img(&form).await?;
    let new_post = post::ActiveModel {
        title: ActiveValue::Set(form.post.title.to_owned()),
        content: ActiveValue::Set(form.post.content.to_owned()),
        author_id: ActiveValue::Set(form.post.author_id.to_owned()),
        img: ActiveValue::Set(img_path),
        ..Default::default()
    }
        .insert(db_client)
        .await;
    match new_post {
        Ok(post) => Ok(post),
        Err(err) => Err(AppError::DbError(AppDbError::from(err))),
    }
}