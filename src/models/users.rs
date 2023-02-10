use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow};
use validator::Validate;

use crate::db::DbPool;

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
    #[validate(length(min = 3, message = "username have to be at least 3 characters long"))]
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
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
