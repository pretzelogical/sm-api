use sea_orm::{
  ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
  QueryFilter, QueryOrder, QuerySelect, ActiveValue, ActiveModelTrait
};
use serde::Serialize;
use sm_entity::post;

use crate::error::{AppDbError, AppError};


#[derive(Serialize)]
pub struct PostResponseItem {
    pub post: sm_entity::post::Model,
    pub comment: Option<Vec<sm_entity::comment::Model>>,
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

pub async fn get_by_id(
  post_id: i64,
  db_client: &DatabaseConnection,
) -> Result<PostResponseItem, AppError> {
  let db_res = post::Entity::find_by_id(post_id).one(db_client).await;
  match db_res {
      Ok(post) => match post {
          Some(post) => Ok(PostResponseItem {
              comment: match get_comments(&post, db_client, None).await {
                  Ok(comment) => comment,
                  Err(error) => {
                      println!("uncaught db error {:#?}", error);
                      None
                  }
              },
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
              let comment = get_comments(&post, db_client, None).await;
              if let Ok(comment) = comment {
                  item_vec.push(PostResponseItem {
                      post: post,
                      comment: comment,
                  });
              } else if let Err(error) = comment {
                  println!("uncaught db error {:#?}", error);
                  item_vec.push(PostResponseItem {
                      post: post,
                      comment: None,
                  });
              }
          }
          Ok(item_vec)
      }
      Err(err) => Err(AppError::DbError(AppDbError::from(err))),
  }
}

pub async fn create_post(
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