use actix_web::{HttpResponse, Responder};
use sea_orm::DbErr;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct JsonError {
    pub error: String
}


impl JsonError {
    pub fn new(error: String) -> JsonError {
        JsonError {
            error
        }
    }
}

impl From<String> for JsonError {
    fn from(error: String) -> JsonError {
        JsonError {
            error
        }
    }
}

impl From<&'static str> for JsonError {
    fn from(error: &'static str) -> JsonError {
        let error: String = error.into();
        JsonError {
            error
        }
    }
}

impl From<DbErr> for JsonError {
    fn from(error: DbErr) -> JsonError {
        let error = error.to_string();
        JsonError {
            error
        }
    }
}

impl From<&DbErr> for JsonError {
    fn from(error: &DbErr) -> JsonError {
        let error = error.to_string();
        JsonError {
            error
        }
    }
}

#[derive(Debug)]
pub struct AppDbError {
    pub error: Option<DbErr>,
    pub message: Option<&'static str>
}

impl AppDbError {
    pub fn new(error: Option<DbErr>, message: Option<&'static str>) -> AppDbError {
        AppDbError {
            error: error,
            message: message
        }
    }
}

impl From<DbErr> for AppDbError {
    fn from(error: DbErr) -> Self {
        AppDbError::new(Some(error), None)
    }
}

impl From<&'static str> for AppDbError {
    fn from(message: &'static str) -> Self {
        AppDbError::new(None, Some(message))
    }
}

#[derive(Debug)]
pub enum AppError {
    DbError(AppDbError),
    BadRequest(&'static str),
    NotFound(&'static str),
    InternalError(&'static str)
}

impl Into<HttpResponse> for AppError {
    fn into(self) -> HttpResponse {
        match self {
            AppError::DbError(db_err) => {
                let error = &db_err.error;
                let message = &db_err.message;
                match (*message, error) {
                    (Some(message), _) => {
                        HttpResponse::InternalServerError()
                            .json(JsonError::from(message))
                    }
                    (None, Some(err)) => {
                        HttpResponse::InternalServerError()
                            .json(JsonError::from(err))
                    },
                    (None, None) => {
                        println!("DbError has no message or DbErr, this should not happen!");
                        HttpResponse::InternalServerError()
                            .json(JsonError::from("Database error"))
                    },
                }
            },
            AppError::BadRequest(message) => HttpResponse::BadRequest()
                .json(JsonError::from(message)),
            AppError::NotFound(message) => HttpResponse::NotFound()
                .json(JsonError::from(message)),
            AppError::InternalError(message) => HttpResponse::InternalServerError()
            .json(JsonError::from(message))
        }
    }
}
