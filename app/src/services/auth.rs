use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::error::AppError;
use crate::services;

const DEV_SECRET: &'static str = "SECRET";

const JWT_CREATE_ERR: &'static str = "Cannot create jwt";
const JWT_CHECK_ERR: &'static str = "Cannot check jwt";


#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    // user id
    sub: i64,
    // Token expiration
    exp: f64
}

// Gets the current time as the time since the unix epoch
fn now() -> Result<Duration, AppError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(exp) => {
            Ok(exp)
        },
        Err(_) => Err(AppError::InternalError(JWT_CREATE_ERR))
    }
}

fn exp() -> Result<Duration, AppError> {
    match now() {
        Ok(now) => Ok(now + Duration::from_secs(7 * 24 * 60 * 60)),
        Err(_) => Err(AppError::InternalError(JWT_CREATE_ERR))
    }
}


// Creates a jwt that logs the user in for 7 days
pub fn create_auth_token(user: &sm_entity::user::Model) -> Result<String, AppError>  {
    let exp = exp()?;
    match encode(
        &Header::default(),
        &AuthClaims {
            sub: user.id,
            exp: exp.as_secs_f64()
        },
        &EncodingKey::from_secret(DEV_SECRET.as_ref())
    ) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(AppError::InternalError(JWT_CREATE_ERR))
    }
}

pub fn is_token_expired(token: &String) -> Result<bool, AppError> {
    let now = now()?;

    match decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(DEV_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256)
    ) {
        Ok(claims) => {
            let user_exp = claims.claims.exp;
            if now.as_secs_f64() > user_exp {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Err(_) => Err(AppError::InternalError("Unable to decode token"))
    }
}


// Checks if the token is valid
pub async fn check_auth_token(token: &String, db_client: &DatabaseConnection) -> Result<sm_entity::user::Model, AppError> {
    let now = now()?;

    match decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(DEV_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256)
    ) {
        Ok(claims) => {
            let user_exp = claims.claims.exp;
            let user_id = claims.claims.sub;
            if now.as_secs_f64() > user_exp {
                return Err(AppError::Unauthorized("Token expired"));
            }
            match services::user::get_by_id(user_id, db_client).await {
                Ok(user) => Ok(user),
                Err(err) => Err(err)
            }
        },
        Err(_) => Err(AppError::InternalError(JWT_CHECK_ERR))
    }
}

// Logs in a user with username and password and returns the user and a jwt
pub async fn login_user(user_name: &String, user_password: &String, db_client: &DatabaseConnection) -> Result<(sm_entity::user::Model, String), AppError> {
    let user = services::user::get_by_login(user_name, user_password, db_client).await?;
    let token = create_auth_token(&user)?;
    Ok((user, token))
}


// Creates a new user with username and password and returns the user and a jwt
pub async fn create_user(user_name: &String, user_password: &String, db_client: &DatabaseConnection) -> Result<(sm_entity::user::Model, String), AppError> {
    let user = services::user::new_user(user_name, user_password, db_client).await?;
    let token = create_auth_token(&user)?;
    Ok((user, token))
}