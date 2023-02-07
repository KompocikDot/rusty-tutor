use crate::errors::ApiError;
use crate::validate::validate_body;
use crate::{
    extractors::jwt::JWTToken,
    models::adverts::AdvertWithUser,
    response::{respond_json},
    AppState,
};
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
}

#[post("/")]
async fn create_advert(
    app_state: web::Data<AppState>,
    data: web::Json<Advert>,
    jwt: JWTToken,
) -> Result<Json<String>, ApiError> {
    validate_body(&data)?;
    data.create(&app_state.db, jwt.id).await?;
    Ok(Json("Successfuly created obj".to_string()))
}

#[get("/{id}")]
async fn get_advert(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<Json<AdvertWithUser>, ApiError> {
    let advert_id = path.into_inner();
    let advert = sqlx::query_as!(
        AdvertWithUser,
        "SELECT
            adverts.id as id, user_id, title, content, username, name, surname
        FROM adverts INNER JOIN users
        ON adverts.user_id = users.id WHERE adverts.id = $1",
        advert_id.0
    )
    .fetch_one(&app_state.db)
    .await?;

    respond_json(advert)
}

#[get("/")]
async fn get_all_adverts(app_state: web::Data<AppState>) -> Result<Json<Vec<AdvertWithUser>>, ApiError> {
    let adverts = sqlx::query_as!(
        AdvertWithUser,
        "SELECT
            adverts.id as id, user_id, title, content, username, name, surname
        FROM adverts INNER JOIN users
        ON adverts.user_id = users.id",
    )
    .fetch_all(&app_state.db)
    .await?;

    respond_json(adverts)
}
