use actix_web::web::Json;
use actix_web::{post, web, Scope};
use pwhash::bcrypt;
use serde::Deserialize;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::users::User;
use crate::response::{respond_json, JWTResponse};
use crate::utils::create_token;
use crate::validate::validate_body;
use crate::AppState;

#[derive(Deserialize, Validate)]
struct LoginBody {
    #[validate(length(
        min = 3,
        max = 64,
        message = "password have to be between 9 and 64 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 9,
        max = 64,
        message = "password have to be between 9 and 64 characters"
    ))]
    pub password: String,
}

pub fn auth_scope() -> Scope {
    web::scope("/auth").service(register).service(login)
}

#[post("/login")]
async fn login(
    app_state: web::Data<AppState>,
    data: web::Json<LoginBody>,
) -> Result<Json<JWTResponse>, ApiError> {
    validate_body(&data)?;
    let user_obj = User::get(&app_state.db, &data.username).await;

    let user = match user_obj {
        Ok(user) => user,
        Err(_) => return Err(ApiError::Unauthorized("bad username or password".into())),
    };

    match bcrypt::verify(&data.password, &user.password) {
        true => {
            let token = create_token(user.id, app_state.secret_key.clone());
            respond_json(JWTResponse { token })
        }
        false => Err(ApiError::Unauthorized("bad username or password".into())),
    }
}

#[post("/register")]
async fn register(
    app_state: web::Data<AppState>,
    data: web::Json<User>,
) -> Result<Json<JWTResponse>, ApiError> {
    validate_body(&data)?;
    let insert_query = data.create(&app_state.db).await?;
    let token = create_token(insert_query.id, app_state.secret_key.clone());
    respond_json(JWTResponse { token })
}
