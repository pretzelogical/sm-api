use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::Display;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonError {
    pub error: String,
}

impl From<String> for JsonError {
    fn from(error: String) -> JsonError {
        JsonError { error }
    }
}

impl From<&'static str> for JsonError {
    fn from(error: &'static str) -> JsonError {
        let error: String = error.into();
        JsonError { error }
    }
}

impl From<DbErr> for JsonError {
    fn from(error: DbErr) -> JsonError {
        let error = error.to_string();
        JsonError { error }
    }
}

impl From<&DbErr> for JsonError {
    fn from(error: &DbErr) -> JsonError {
        let error = error.to_string();
        JsonError { error }
    }
}

#[derive(Debug, Display)]
pub enum AppError {
    DbError(DbErr),
    BadRequest(&'static str),
    NotFound(&'static str),
    InternalError(&'static str),
    Unauthorized(&'static str),
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            AppError::DbError(error) => {
                HttpResponse::build(self.status_code()).json(JsonError::from(error.to_string()))
            }
            AppError::BadRequest(message) => {
                HttpResponse::build(self.status_code()).json(JsonError::from(message.to_string()))
            }
            AppError::NotFound(message) => {
                HttpResponse::build(self.status_code()).json(JsonError::from(message.to_string()))
            }
            AppError::InternalError(message) => {
                HttpResponse::build(self.status_code()).json(JsonError::from(message.to_string()))
            }
            AppError::Unauthorized(message) => {
                HttpResponse::build(self.status_code()).json(JsonError::from(message.to_string()))
            }
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
