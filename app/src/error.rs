use actix_web::HttpResponse;
use derive_more::derive::{Display, Error};
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

// #[derive(Debug, Display)]
// #[display("error: {:#?}", message)]
// pub struct AppDbError {
//     pub error: Option<DbErr>,
//     pub message: Option<&'static str>,
// }
//
// impl AppDbError {
//     pub fn new(error: Option<DbErr>, message: Option<&'static str>) -> AppDbError {
//         AppDbError {
//             error: error,
//             message: message,
//         }
//     }
// }
//
// impl From<DbErr> for AppDbError {
//     fn from(error: DbErr) -> Self {
//         AppDbError::new(Some(error), None)
//     }
// }
//
// impl From<&'static str> for AppDbError {
//     fn from(message: &'static str) -> Self {
//         AppDbError::new(None, Some(message))
//     }
// }

#[derive(Debug, Error, Display)]
pub enum AppError {
    DbError(DbErr),
    BadRequest(&'static str),
    NotFound(&'static str),
    InternalError(&'static str),
    Unauthorized(&'static str),
}

impl Into<HttpResponse> for AppError {
    fn into(self) -> HttpResponse {
        match self {
            AppError::DbError(error) => {
                HttpResponse::InternalServerError().json(JsonError::from(error.to_string()))
            }
            AppError::BadRequest(message) => {
                HttpResponse::BadRequest().json(JsonError::from(message))
            }
            AppError::NotFound(message) => HttpResponse::NotFound().json(JsonError::from(message)),
            AppError::InternalError(message) => {
                HttpResponse::InternalServerError().json(JsonError::from(message))
            }
            AppError::Unauthorized(message) => {
                HttpResponse::Unauthorized().json(JsonError::from(message))
            }
        }
    }
}
