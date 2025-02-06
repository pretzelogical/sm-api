use crate::error::AppError;
use crate::routes::prelude::*;
use crate::services;

pub async fn get_by_handle(
    app_state: web::Data<AppState>,
    handle: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let db_client = &app_state.db_client;
    let user = services::user::get_by_handle(handle.into_inner(), db_client).await?;
    Ok(HttpResponse::Ok().json(user))
}

// #[delete("/user")]
// pub async fn create_user(app_state: web::Data<AppState>, user_args: web::Json<CreateUserArgs>) -> impl Responder {

// }
