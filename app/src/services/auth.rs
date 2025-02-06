use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use services::time::{exp, now};
use services::user::NewUserArgs;

use crate::error::AppError;
use crate::services;

const DEV_SECRET: &str = "SECRET";

const JWT_CREATE_ERR: &str = "Cannot create jwt";
const JWT_CHECK_ERR: &str = "Cannot check jwt";

#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    // user id
    sub: i64,
    // Token expiration
    exp: f64,
}

// Creates a jwt that logs the user in for 7 days
pub fn create_auth_token(user: &sm_entity::user::Model) -> Result<String, AppError> {
    let exp = exp()?;
    match encode(
        &Header::default(),
        &AuthClaims {
            sub: user.id,
            exp: exp.as_secs_f64(),
        },
        &EncodingKey::from_secret(DEV_SECRET.as_ref()),
    ) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(AppError::InternalError(JWT_CREATE_ERR)),
    }
}

pub fn is_token_expired(token: &str) -> Result<bool, AppError> {
    let now = now()?;

    match decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(DEV_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => {
            let user_exp = claims.claims.exp;
            if now.as_secs_f64() > user_exp {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Err(AppError::InternalError("Unable to decode token")),
    }
}

// Checks if the token is valid
pub async fn check_auth_token(
    token: &str,
    db_client: &DatabaseConnection,
) -> Result<sm_entity::user::Model, AppError> {
    let now = now()?;

    match decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(DEV_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => {
            let user_exp = claims.claims.exp;
            let user_id = claims.claims.sub;
            if now.as_secs_f64() > user_exp {
                return Err(AppError::Unauthorized("Token expired"));
            }
            match services::user::get_by_id(user_id, db_client).await {
                Ok(user) => Ok(user),
                Err(err) => Err(err),
            }
        }
        Err(_) => Err(AppError::InternalError(JWT_CHECK_ERR)),
    }
}

// Logs in a user with username and password and returns the user and a jwt
pub async fn login_user(
    user_name: &String,
    user_password: &String,
    db_client: &DatabaseConnection,
) -> Result<(sm_entity::user::Model, String), AppError> {
    let user = services::user::get_by_login(user_name, user_password, db_client).await?;
    let token = create_auth_token(&user)?;
    Ok((user, token))
}

// Creates a new user with username and password and returns the user and a jwt
pub async fn create_user(
    args: NewUserArgs,
    db_client: &DatabaseConnection,
) -> Result<(sm_entity::user::Model, String), AppError> {
    let user = services::user::new_user(args, db_client).await?;
    let token = create_auth_token(&user)?;
    Ok((user, token))
}
