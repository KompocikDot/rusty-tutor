use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow};
use validator::Validate;

use crate::types::DbPool;

#[derive(FromRow)]
pub struct UserCreate {
    pub id: i32,
}

#[derive(FromRow, Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub id: i32,
    #[validate(length(min = 3, max = 64, message = "username must be between 5 and 64 characters long"))]
    pub username: String,
    #[serde(skip_serializing)]
    #[validate(custom(
        function = "crate::validate::validate_password",
        message = "Password should contain at least 1 digit, 1 special character, 1 uppercase character and be at least 8 characters long"
    ))]
    pub password: String,

    #[validate(length(min = 2, max = 64, message = "name must be between 5 and 64 characters"))]
    pub name: String,
    #[validate(length(min = 2, max = 64, message = "surname must be between 5 and 64 characters"))]
    pub surname: String,
}

impl User {
    pub async fn create(&self, db: &DbPool) -> sqlx::Result<UserCreate> {
        let pwd = pwhash::bcrypt::hash(self.password.clone()).unwrap();
        sqlx::query_as!(
            UserCreate,
            "INSERT INTO users(username, password, name, surname) VALUES($1, $2, $3, $4) RETURNING id",
            self.username.to_lowercase(),
            pwd,
            self.name,
            self.surname,
        )
            .fetch_one(db)
            .await
    }

    pub async fn get_by_id(db: &DbPool, user_id: i32) -> sqlx::Result<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(db)
            .await
    }

    pub async fn get(db: &DbPool, username: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            username.to_lowercase()
        )
        .fetch_one(db)
        .await
    }

    pub async fn update(&self, db: &DbPool, user_id: i32) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            "UPDATE users SET username = $1, name = $2, surname = $3 WHERE id = $4 RETURNING *",
            self.username,
            self.name,
            self.surname,
            user_id,
        )
        .fetch_one(db)
        .await
    }

    pub async fn delete(db: &DbPool, user_id: i32) -> sqlx::Result<PgQueryResult> {
        sqlx::query!("DELETE FROM users where id = $1", user_id)
            .execute(db)
            .await
    }
}
