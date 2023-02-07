use actix_web::{web::Json};
use serde::Serialize;

use crate::errors::{ApiError};

#[derive(Serialize)]
pub struct Response<'a> {
    pub message: &'a str,
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