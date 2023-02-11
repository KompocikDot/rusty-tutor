use actix_web::{http::StatusCode, web::Json, HttpResponse};
use serde::Serialize;

use crate::errors::ApiError;

#[derive(Serialize)]
pub struct Response<T> {
    pub data: T
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