use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::error::AppError;


// Gets the current time as the time since the unix epoch
pub fn now() -> Result<Duration, AppError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(exp) => {
            Ok(exp)
        },
        Err(_) => Err(AppError::InternalError("Cannot get timestamp"))
    }
}

pub fn exp() -> Result<Duration, AppError> {
    match now() {
        Ok(now) => Ok(now + Duration::from_secs(7 * 24 * 60 * 60)),
        Err(_) => Err(AppError::InternalError("Cannot get timestamp"))
    }
}