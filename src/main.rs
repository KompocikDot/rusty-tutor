mod errors;
mod extractors;
mod models;
mod response;
mod scopes;
mod validate;
mod utils;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use env_logger::Env;
use scopes::{adverts::adverts_scope, auth::auth_scope, user::user_scope};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    secret_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let secret_key = dotenv::var("SECRET").unwrap();
    let db_url = env!("DATABASE_URL");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap();

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
