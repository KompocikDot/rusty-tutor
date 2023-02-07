use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use validator::Validate;

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
    pub async fn create(&self, db: &Pool<Postgres>, user_id: i32) -> sqlx::Result<Advert> {
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
}
