use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::Deserialize;
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

pub async fn get_by_handle(
    handle: String,
    db_client: &DatabaseConnection,
) -> Result<user::Model, AppError> {
    let handle = handle.replace("@", "");
    let db_res = user::Entity::find()
        .filter(user::Column::Handle.contains(handle))
        .one(db_client)
        .await;
    match db_res {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(AppError::NotFound("User with handle not found")),
        Err(err) => Err(AppError::DbError(err)),
    }
}

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

#[derive(Deserialize)]
pub struct NewUserArgs {
    pub name: String,
    pub pass: String,
    pub handle: String,
}

// Creates a new user
pub async fn new_user(
    args: NewUserArgs,
    db_client: &DatabaseConnection,
) -> Result<user::Model, AppError> {
    let new_user = user::ActiveModel {
        name: ActiveValue::Set(args.name.to_owned()),
        pass: ActiveValue::Set(args.pass.to_owned()),
        handle: ActiveValue::Set(args.handle.to_owned()),
        ..Default::default()
    }
    .insert(db_client)
    .await;
    match new_user {
        Ok(new_user) => Ok(new_user),
        Err(err) => Err(AppError::DbError(err)),
    }
}
