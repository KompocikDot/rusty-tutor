use sqlx::{postgres::PgPoolOptions};

use crate::types::DbPool;

pub async fn create_pool(url: &str) -> DbPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .unwrap()
}
