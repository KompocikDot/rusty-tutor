mod db;
mod errors;
mod extractors;
mod handlers;
mod models;
mod response;
mod utils;
mod validate;
pub mod types;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};

use env_logger::Env;
use handlers::{adverts::adverts_scope, auth::auth_scope, user::user_scope, opinions::opinions_scope};
use types::DbPool;

#[derive(Clone)]
pub struct AppState {
    db: DbPool,
    secret_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let secret_key = dotenv::var("SECRET").unwrap();
    let db_url = env!("DATABASE_URL");

    let pool: DbPool = db::create_pool(db_url).await;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_data = AppState {
        db: pool,
        secret_key,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allow_any_header()
                    .allow_any_method(),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(auth_scope())
            .service(adverts_scope())
            .service(user_scope())
            .service(opinions_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
