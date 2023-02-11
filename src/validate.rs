use actix_web::web::Json;
use validator::{Validate, ValidationError};

use crate::errors::ApiError;

pub fn validate_body<T>(params: &Json<T>) -> Result<(), ApiError>
where
    T: Validate,
{
    match params.validate() {
        Ok(()) => Ok(()),
        Err(errors) => Err(ApiError::ValidationError(errors)),
    }
}

pub fn validate_password(pwd: &String) -> Result<(), ValidationError> {
    let mut has_special_char = false;
    let mut has_digit = false;
    let mut has_uppercase = false;

    for char in pwd.chars() {
        has_special_char |= char.is_ascii_punctuation();
        has_digit |= char.is_ascii_digit();
        has_uppercase |= char.is_uppercase();

        if has_special_char && has_digit && has_uppercase {
            break;
        }
    }

    if has_special_char && has_digit && has_uppercase && pwd.len() >= 8 {
        return Ok(());
    }
    Err(ValidationError::new("invalid passport"))
}
