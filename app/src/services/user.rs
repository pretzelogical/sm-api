use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sm_entity::user;

use crate::error::AppError;

pub async fn get_by_id(
    user_id: i64,
    db_client: &DatabaseConnection,
) -> Result<user::Model, AppError> {
    let db_res = user::Entity::find_by_id(user_id).one(db_client).await;
    match db_res {
        Ok(user) => match user {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound("User with id not found")),
        },
        Err(err) => Err(AppError::DbError(err)),
    }
}

// pub async fn get_by_name(
//     user_name: &String,
//     db_client: &DatabaseConnection,
// ) -> Result<user::Model, AppError> {
//     let db_res = user::Entity::find()
//         .filter(user::Column::Name.contains(user_name))
//         .one(db_client)
//         .await;
//     match db_res {
//         Ok(Some(user)) => Ok(user),
//         Ok(None) => Err(AppError::NotFound("User with name not found")),
//         Err(err) => Err(AppError::DbError(err)),
//     }
// }

// Finds the user with the matching name AND password
pub async fn get_by_login(
    user_name: &String,
    user_password: &String,
    db_client: &DatabaseConnection,
) -> Result<user::Model, AppError> {
    let db_res = user::Entity::find()
        .filter(user::Column::Name.contains(user_name))
        .filter(user::Column::Pass.contains(user_password))
        .one(db_client)
        .await;
    match db_res {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(AppError::NotFound("User with name not found")),
        Err(err) => Err(AppError::DbError(err)),
    }
}

// Creates a new user
pub async fn new_user(
    user_name: &String,
    user_password: &String,
    db_client: &DatabaseConnection,
) -> Result<user::Model, AppError> {
    let new_user = user::ActiveModel {
        name: ActiveValue::Set(user_name.to_owned()),
        pass: ActiveValue::Set(user_password.to_owned()),
        ..Default::default()
    }
    .insert(db_client)
    .await;
    match new_user {
        Ok(new_user) => Ok(new_user),
        Err(err) => Err(AppError::DbError(err)),
    }
}
