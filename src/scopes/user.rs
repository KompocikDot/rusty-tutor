use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, patch, web, HttpResponse, Scope};



use crate::errors::ApiError;
use crate::models::users::User;
use crate::response::{respond_json};
use crate::validate::validate_body;
use crate::{extractors::jwt::JWTToken, AppState};

pub fn user_scope() -> Scope {
    Scope::new("/user")
        .service(user_opinions)
        .service(user_profile)
        .service(update_user_profile)
        .service(delete_user_profile)
}

#[get("/opinions/")]
async fn user_opinions(_app_state: web::Data<AppState>) -> HttpResponse {
    todo!();
}

#[get("/profile")]
async fn user_profile(app_state: web::Data<AppState>, jwt: JWTToken) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", jwt.id)
        .fetch_one(&app_state.db)
        .await?;
    respond_json(user)
}

#[patch("/profile")]
async fn update_user_profile(
    state: web::Data<AppState>,
    jwt: JWTToken,
    data: web::Json<User>,
) -> Result<Json<User>, ApiError> {
    validate_body(&data)?;
    let user = data.update(&state.db, jwt.id).await?;
    respond_json(user)
}

#[delete("/profile")]
async fn delete_user_profile(
    state: web::Data<AppState>,
    jwt: JWTToken,
) -> Result<HttpResponse, ApiError> {
    let deleted = User::delete(&state.db, jwt.id).await?;
    if deleted.rows_affected() > 0 {
        Ok(HttpResponse::new(StatusCode::ACCEPTED))
    } else {
        let msg = format!("could not find a user with id: {}", jwt.id);
        Err(ApiError::NotFound(msg))
    }
    
}
