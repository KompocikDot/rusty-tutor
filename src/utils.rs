use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::extractors::jwt::Claims;

pub fn create_token(user_id: i32, secret: String) -> String {
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims = Claims { id: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
