#![allow(unused_imports)]

pub use actix_web::{get, post, put, patch, delete, web};
pub use actix_web::{HttpResponse, Responder};
pub use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
pub use serde::{Deserialize, Serialize};
pub use crate::error::{AppError, JsonError, AppDbError};
pub use crate::AppState;
pub use sm_entity::{user, post};
