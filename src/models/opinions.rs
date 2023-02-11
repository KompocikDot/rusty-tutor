use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::{errors::ApiError, types::DbPool};

#[derive(FromRow, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Opinion {
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(skip_deserializing)]
    pub evaluated_person_id: i32,
    #[serde(skip_deserializing)]
    pub opinion_creator_id: i32,
    pub advert_id: i32,
    pub content: String,
    pub rating: i32,
}

impl Opinion {
    pub async fn is_owner(db: &DbPool, user_id: i32, opinion_id: i32) -> sqlx::Result<bool> {
        let query = sqlx::query!(
            "SELECT opinion_creator_id from opinions where id = $1",
            opinion_id
        )
        .fetch_one(db)
        .await?;
        Ok(query.opinion_creator_id == user_id)
    }

    pub async fn create(&self, db: &DbPool, creator_id: i32) -> sqlx::Result<Opinion> {
        sqlx::query_as!(
            Opinion,
            "INSERT INTO opinions(
                evaluated_person_id, opinion_creator_id, advert_id, content, rating
            ) values($1, $2, $3, $4, $5) RETURNING *",
            self.evaluated_person_id,
            creator_id,
            self.advert_id,
            self.content,
            self.rating
        )
        .fetch_one(db)
        .await
    }

    pub async fn delete(db: &DbPool, user_id: i32, opinion_id: i32) -> Result<(), ApiError> {
        if Opinion::is_owner(db, user_id, opinion_id).await? {
            sqlx::query!("DELETE FROM opinions WHERE id = $1", opinion_id)
                .execute(db)
                .await?;
            Ok(())
        } else {
            Err(ApiError::Unauthorized(
                "You must be a resource owner to do this action".to_string(),
            ))
        }
    }

    pub async fn get_all(db: &DbPool, user_id: i32) -> sqlx::Result<Vec<Opinion>> {
        sqlx::query_as!(
            Opinion,
            "SELECT * FROM opinions where evaluated_person_id = $1",
            user_id
        )
        .fetch_all(db)
        .await
    }
}
