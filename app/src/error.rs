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


#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    BadRequest,
    NotFound,
}
