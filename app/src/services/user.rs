use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use sm_entity::user;

use crate::error::{AppDbError, AppError};


pub async fn get_by_id(user_id: i64, db_client: &DatabaseConnection) -> Result<user::Model, AppError> {
  let db_res = user::Entity::find_by_id(user_id)
      .one(db_client)
      .await;
  match db_res {
      Ok(user) => {
          match user {
              Some(user) => Ok(user),
              None => Err(AppError::NotFound("User with id not found"))
          }
      }
      Err(err) => Err(AppError::DbError(AppDbError::from(err)))
  }
}

pub async fn get_by_name(user_name: &String, db_client: &DatabaseConnection) -> Result<user::Model, AppError> {
  let db_res = user::Entity::find()
      .filter(user::Column::Name.contains(user_name))
      .one(db_client)
      .await;
  match db_res {
      Ok(Some(user)) => Ok(user),
      Ok(None) => Err(AppError::NotFound("User with name not found")),
      Err(err) => Err(AppError::DbError(AppDbError::from(err)))
  }
}