use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(url: &str) -> DbPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .unwrap()
}
