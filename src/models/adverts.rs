use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::{errors::ApiError, types::DbPool};

#[derive(FromRow, Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Advert {
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdvertWithUser {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub username: String,
    pub name: String,
    pub surname: String,
}

impl Advert {
    pub async fn is_owner(db: &DbPool, user_id: i32, advert_id: i32) -> sqlx::Result<bool> {
        let query = sqlx::query!("SELECT user_id from adverts where id = $1", advert_id)
            .fetch_one(db)
            .await?;
        Ok(query.user_id == user_id)
    }

    pub async fn create(&self, db: &DbPool, user_id: i32) -> sqlx::Result<Advert> {
        sqlx::query_as!(
            Advert,
            "INSERT INTO adverts(user_id, title, content) values($1, $2, $3) RETURNING *",
            user_id,
            self.title,
            self.content
        )
        .fetch_one(db)
        .await
    }

    pub async fn delete(db: &DbPool, advert_id: i32, user_id: i32) -> Result<(), ApiError> {
        let is_owner = Advert::is_owner(db, user_id, advert_id).await?;
        if is_owner {
            sqlx::query!(
                "DELETE FROM adverts WHERE user_id = $1 AND id = $2",
                user_id,
                advert_id
            )
            .execute(db)
            .await?;
            Ok(())
        } else {
            Err(ApiError::Unauthorized(
                "You must be a resource owner to do this action".to_string(),
            ))
        }
    }

    pub async fn update(
        &self,
        db: &DbPool,
        advert_id: i32,
        user_id: i32,
    ) -> Result<Advert, ApiError> {
        let is_owner = Advert::is_owner(db, user_id, advert_id).await?;
        if is_owner {
            let advert = sqlx::query_as!(
                Advert,
                "UPDATE adverts SET title = $1, content = $2 WHERE id = $3 RETURNING *",
                self.title,
                self.content,
                advert_id
            )
            .fetch_one(db)
            .await?;

            Ok(advert)
        } else {
            Err(ApiError::Unauthorized(
                "You must be a resource owner to do this action".to_string(),
            ))
        }
    }
}
