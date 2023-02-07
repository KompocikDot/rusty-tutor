use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, Pool, Postgres};
use validator::Validate;

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
    pub async fn create(
        &self,
        db: &Pool<Postgres>,
        creator_id: i32,
        advert_id: i32,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO opinions(
                evaluated_person_id, opinion_creator_id, advert_id, content, rating
            ) values($1, $2, $3, $4, $5)",
            self.evaluated_person_id,
            creator_id,
            self.advert_id,
            self.content,
            self.rating
        )
        .execute(db)
        .await
    }
}
