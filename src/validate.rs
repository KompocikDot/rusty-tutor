use actix_web::web::Json;
use validator::Validate;

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
