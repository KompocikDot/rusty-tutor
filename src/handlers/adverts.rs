use crate::errors::ApiError;
use crate::extractors::path::Item;
use crate::response::respond_accepted;
use crate::validate::validate_body;
use crate::{
    extractors::jwt::JWTToken, models::adverts::AdvertWithUser, response::respond_json, AppState,
};
use actix_web::{delete, patch, HttpResponse};
use actix_web::{
    get, post,
    web::{self, Json},
    Scope,
};

use crate::models::adverts::Advert;

pub fn adverts_scope() -> Scope {
    Scope::new("/adverts")
        .service(create_advert)
        .service(get_advert)
        .service(get_all_adverts)
        .service(update_advert)
}

#[post("/")]
async fn create_advert(
    state: web::Data<AppState>,
    data: web::Json<Advert>,
    jwt: JWTToken,
) -> Result<Json<Advert>, ApiError> {
    validate_body(&data)?;
    let advert = data.create(&state.db, jwt.id).await?;
    respond_json(advert)
}

#[delete("/{id}")]
async fn delete_advert(
    state: web::Data<AppState>,
    jwt: JWTToken,
    advert_id: web::Path<Item>,
) -> Result<HttpResponse, ApiError> {
    Advert::delete(&state.db, advert_id.id, jwt.id).await?;
    respond_accepted()
}

#[get("/{id}")]
async fn get_advert(
    state: web::Data<AppState>,
    path: web::Path<Item>,
) -> Result<Json<AdvertWithUser>, ApiError> {
    let advert_id = path.id;
    let advert = sqlx::query_as!(
        AdvertWithUser,
        "SELECT
            adverts.id as id, user_id, title, content, username, name, surname
        FROM adverts INNER JOIN users
        ON adverts.user_id = users.id WHERE adverts.id = $1",
        advert_id
    )
    .fetch_one(&state.db)
    .await?;

    respond_json(advert)
}

#[get("/")]
async fn get_all_adverts(
    state: web::Data<AppState>,
) -> Result<Json<Vec<AdvertWithUser>>, ApiError> {
    let adverts = sqlx::query_as!(
        AdvertWithUser,
        "SELECT
            adverts.id as id, user_id, title, content, username, name, surname
        FROM adverts INNER JOIN users
        ON adverts.user_id = users.id",
    )
    .fetch_all(&state.db)
    .await?;

    respond_json(adverts)
}

#[patch("/{id}")]
async fn update_advert(
    state: web::Data<AppState>,
    data: web::Json<Advert>,
    path: web::Path<Item>,
    jwt: JWTToken,
) -> Result<Json<Advert>, ApiError> {
    validate_body(&data)?;
    let advert = data.update(&state.db, path.id, jwt.id).await?;
    respond_json(advert)
}
