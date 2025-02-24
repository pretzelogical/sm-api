use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, SqlErr,
};
use serde::Serialize;
use sm_entity::{comment, post};

use crate::error::AppError;
use crate::routes::post::CreatePostForm;
use crate::services::image::upload_post_img;

use super::time::now;

#[derive(Serialize)]
pub struct PostResponseItem {
    pub post: sm_entity::post::Model,
    pub comment: Option<Vec<sm_entity::comment::Model>>,
    pub tag: Option<Vec<sm_entity::tag::Model>>,
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
            if !comments.is_empty() {
                Ok(Some(comments))
            } else {
                Ok(None)
            }
        }
        Err(error) => Err(AppError::DbError(error)),
    }
}

pub async fn get_tags(
    post: &sm_entity::post::Model,
    db_client: &DatabaseConnection,
) -> Result<Option<Vec<sm_entity::tag::Model>>, AppError> {
    match post
        .find_related(sm_entity::tag::Entity)
        .all(db_client)
        .await
    {
        Ok(tags) => {
            if !tags.is_empty() {
                Ok(Some(tags))
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(AppError::DbError(err)),
    }
}

pub async fn get_num_likes(
    post: &sm_entity::post::Model,
    db_client: &DatabaseConnection,
) -> Result<u64, AppError> {
    match post
        .find_related(sm_entity::like::Entity)
        .count(db_client)
        .await
    {
        Ok(num_likes) => Ok(num_likes),
        Err(err) => Err(AppError::DbError(err)),
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
                post,
            }),
            None => Err(AppError::NotFound("post not found")),
        },
        Err(err) => Err(AppError::DbError(err)),
    }
}

pub async fn get_latest(
    db_client: &DatabaseConnection,
    page: i64,
) -> Result<Vec<PostResponseItem>, AppError> {
    let _page = if page > 0 { page } else { 1 };
    match post::Entity::find()
        .order_by_asc(post::Column::Date)
        .limit(10)
        .find_with_related(comment::Entity)
        .all(db_client)
        .await
    {
        Ok(posts_comments) => {
            let mut posts_vec = Vec::new();
            for (post, comment) in posts_comments {
                let comment = if comment.is_empty() {
                    None
                } else {
                    Some(comment)
                };
                posts_vec.push(PostResponseItem {
                    tag: get_tags(&post, db_client).await?,
                    post,
                    comment,
                });
            }
            Ok(posts_vec)
        }
        Err(error) => Err(AppError::DbError(error)),
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
                item_vec.push(PostResponseItem {
                    comment: get_comments(&post, db_client, None).await?,
                    tag: get_tags(&post, db_client).await?,
                    post,
                });
            }
            Ok(item_vec)
        }
        Err(err) => Err(AppError::DbError(err)),
    }
}

pub async fn create_tags(
    form: CreatePostForm,
    post: &post::Model,
    db_client: &DatabaseConnection,
) -> Result<Option<Vec<sm_entity::tag::Model>>, AppError> {
    let tags = &form.post.tags;
    if let Some(tags) = tags {
        if tags.is_empty() {
            return Ok(None);
        }
        let mut new_tags_vec: Vec<sm_entity::tag::Model> = Vec::with_capacity(tags.len());
        for tag in tags {
            let new_tag = sm_entity::tag::ActiveModel {
                name: ActiveValue::Set(tag.to_owned()),
                ..Default::default()
            }
            .insert(db_client)
            .await;
            match new_tag {
                Ok(tag) => new_tags_vec.push(tag),
                Err(error) => match &error.sql_err() {
                    // If tag already exists then find it and add it
                    Some(SqlErr::UniqueConstraintViolation(_)) => {
                        match sm_entity::tag::Entity::find()
                            .filter(sm_entity::tag::Column::Name.contains(tag))
                            .order_by_asc(sm_entity::tag::Column::Name)
                            .one(db_client)
                            .await
                        {
                            Ok(Some(tag)) => new_tags_vec.push(tag),
                            Ok(None) => return Err(AppError::InternalError("Cannot find tag")),
                            Err(err) => return Err(AppError::DbError(err)),
                        }
                    }
                    Some(_) => return Err(AppError::DbError(error)),
                    None => (),
                },
            }
        }
        // Junction table
        for tag in &new_tags_vec {
            let new_post_tag = sm_entity::post_tag::ActiveModel {
                post_id: ActiveValue::Set(post.id),
                tag_id: ActiveValue::Set(tag.id),
                ..Default::default()
            }
            .insert(db_client)
            .await;
            match new_post_tag {
                Ok(_) => (),
                Err(error) => return Err(AppError::DbError(error)),
            }
        }
        Ok(Some(new_tags_vec))
    } else {
        Ok(None)
    }
}

pub async fn create_post(
    form: CreatePostForm,
    db_client: &DatabaseConnection,
) -> Result<post::Model, AppError> {
    let img_path = upload_post_img(&form).await?;
    let now = now()?;
    let new_post = post::ActiveModel {
        title: ActiveValue::Set(form.post.title.to_owned()),
        content: ActiveValue::Set(form.post.content.to_owned()),
        author_id: ActiveValue::Set(form.post.author_id.to_owned()),
        img: ActiveValue::Set(img_path),
        date: ActiveValue::Set(now.as_secs_f64()),
        ..Default::default()
    }
    .insert(db_client)
    .await;

    match new_post {
        Ok(post) => {
            let _ = create_tags(form, &post, db_client).await?;
            Ok(post)
        }
        Err(err) => Err(AppError::DbError(err)),
    }
}
