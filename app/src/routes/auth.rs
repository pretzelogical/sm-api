use actix_web::HttpRequest;
use serde::Serialize;

use crate::routes::prelude::*;
use crate::services;

#[derive(Serialize)]
pub struct AuthUserResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

impl AuthResponse {
    pub fn new(token: String, user: sm_entity::user::Model) -> AuthResponse {
        AuthResponse {
            token,
            user: AuthUserResponse {
                id: user.id,
                name: user.name,
            },
        }
    }
}

// Check if user is session authenticated
pub async fn check_auth(
    app_state: web::Data<AppState>,
    args: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let db_client = &app_state.db_client;
    let token = match args.headers().get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string().replace("Bearer ", ""),
        None => return Err(AppError::Unauthorized("Token missing")),
    };
    let user = services::auth::check_auth_token(&token, db_client).await?;
    Ok(HttpResponse::Ok().json(AuthResponse::new(token, user)))
}

#[derive(Deserialize)]
pub struct AuthArgs {
    pub username: String,
    pub password: String,
}

// Creates a new session from a user
pub async fn log_in(
    app_state: web::Data<AppState>,
    args: web::Json<AuthArgs>,
) -> Result<HttpResponse, AppError> {
    let db_client = &app_state.db_client;
    let (user, token) =
        services::auth::login_user(&args.username, &args.password, db_client).await?;
    Ok(HttpResponse::Ok().json(AuthResponse::new(token, user)))
}

// Creates a new user and returns a jwt
pub async fn register(
    app_state: web::Data<AppState>,
    args: web::Json<AuthArgs>,
) -> Result<HttpResponse, AppError> {
    let db_client = &app_state.db_client;
    let (user, token) =
        services::auth::create_user(&args.username, &args.password, db_client).await?;
    Ok(HttpResponse::Ok().json(AuthResponse::new(token, user)))
}
