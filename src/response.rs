use actix_web::{http::StatusCode, web::Json, HttpResponse};
use serde::Serialize;

use crate::errors::ApiError;

pub type APIResponse<T> = Result<T, ApiError>;

#[derive(Serialize)]
pub struct Response<'m> {
    pub message: &'m str,
}

#[derive(Serialize)]
pub struct JWTResponse {
    pub token: String,
}

pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_accepted() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}
