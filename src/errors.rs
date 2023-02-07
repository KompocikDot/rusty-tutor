use actix_web::{
    error::{ResponseError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                let resp: ErrorResponse = error.into();
                HttpResponse::BadRequest().json(resp)
            }
            ApiError::NotFound(message) => {
                let resp: ErrorResponse = message.into();
                HttpResponse::NotFound().json(resp)
            }
            ApiError::ValidationError(errors) => {
                let resp: ErrorResponse = errors.to_vec().into();
                HttpResponse::UnprocessableEntity().json(resp)
            }
            ApiError::Unauthorized(error) => {
                let resp: ErrorResponse = error.into();
                HttpResponse::Unauthorized().json(resp)
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ApiErrors
impl From<sqlx::error::Error> for ApiError {
    fn from(error: sqlx::error::Error) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            sqlx::Error::RowNotFound => ApiError::NotFound("could not find row".into()),
            _ => ApiError::InternalServerError("".into())
        }
    }
}