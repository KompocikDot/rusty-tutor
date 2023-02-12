use actix_web::{error::{ResponseError, PathError}, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    JSONParseError(String),
    NotFound(String),
    PathParseError(String),
    ValidationError(ValidationErrors),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    errors: ValidationErrors,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) |
            ApiError::PathParseError(error) |
            ApiError::JSONParseError(error)
            => {
                let resp: ErrorResponse = error.into();
                HttpResponse::BadRequest().json(resp)
            }
            ApiError::NotFound(message) => {
                let resp: ErrorResponse = message.into();
                println!("Test");
                HttpResponse::NotFound().json(resp)
            }
            ApiError::ValidationError(errors) => {
                let resp: ValidationErrorResponse = errors.to_owned().into();
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
impl From<ValidationErrors> for ValidationErrorResponse {
    fn from(errors: ValidationErrors) -> Self {
        ValidationErrorResponse { errors }
    }
}

/// Convert sqlx::error::Error to ApiErrors
impl From<sqlx::error::Error> for ApiError {
    fn from(error: sqlx::error::Error) -> ApiError {
        match error {
            sqlx::Error::RowNotFound => ApiError::NotFound("Data not found in db".into()),
            // TODO: Change it later to not display stringified dberrors to the user
            err => ApiError::InternalServerError(err.to_string()),
        }
    }
}