use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::Error as ActixWebError;
use actix_web::{web, FromRequest};
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, errors::Error as JwtError, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;
use crate::AppState;
pub struct JWTToken {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

impl FromRequest for JWTToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &actix_web::HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = _req.clone();
        let authorization_header_option: Option<&HeaderValue> =
            req.headers().get(actix_web::http::header::AUTHORIZATION);

        // No Header was sent
        if authorization_header_option.is_none() {
            return ready(Err(ApiError::Unauthorized(
                "No authentication token sent!".to_string(),
            )
            .into()));
        }
        let authentication_token: String = authorization_header_option
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string();
        if authentication_token.is_empty() {
            return ready(Err(ApiError::Unauthorized(
                "Authentication token has foreign chars!".to_string(),
            )
            .into()));
        }
        let secret: &String = &req.app_data::<web::Data<AppState>>().unwrap().secret_key;

        let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &authentication_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match token_result {
            Ok(token) => ready(Ok(JWTToken {
                id: token.claims.id,
            })),
            Err(_e) => ready(Err(ApiError::Unauthorized(
                "Invalid authentication token sent!".to_string(),
            )
            .into())),
        }
    }
}
