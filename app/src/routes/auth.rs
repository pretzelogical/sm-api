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
pub async fn check_auth(app_state: web::Data<AppState>, args: HttpRequest) -> impl Responder {
    let db_client = &app_state.db_client;
    let token = match args.headers().get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string().replace("Bearer ", ""),
        None => return AppError::Unauthorized("Token missing").into(),
    };
    match services::auth::check_auth_token(&token, db_client).await {
        Ok(user) => HttpResponse::Ok().json(AuthResponse::new(token, user)),
        Err(err) => err.into(),
    }
}

#[derive(Deserialize)]
pub struct AuthArgs {
    pub username: String,
    pub password: String,
}

// Creates a new session from a user
pub async fn log_in(app_state: web::Data<AppState>, args: web::Json<AuthArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    match services::auth::login_user(&args.username, &args.password, db_client).await {
        Ok((user, token)) => HttpResponse::Ok().json(AuthResponse::new(token, user)),
        Err(err) => err.into(),
    }
}

// Creates a new user and returns a jwt
pub async fn register(app_state: web::Data<AppState>, args: web::Json<AuthArgs>) -> impl Responder {
    let db_client = &app_state.db_client;
    match services::auth::create_user(&args.username, &args.password, db_client).await {
        Ok((user, token)) => HttpResponse::Ok().json(AuthResponse::new(token, user)),
        Err(err) => err.into(),
    }
}

