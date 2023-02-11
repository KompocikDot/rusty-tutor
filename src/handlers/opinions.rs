use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpResponse, Scope,
};

use crate::{
    extractors::{jwt::JWTToken, path::Item},
    models::opinions::Opinion,
    response::{respond_accepted, respond_json},
    types::APIResponse,
    validate::validate_body,
    AppState,
};

pub fn opinions_scope() -> Scope {
    Scope::new("/opinions")
        .service(create_opinion)
        .service(get_all_opinions)
}

#[post("/")]
async fn create_opinion(
    state: web::Data<AppState>,
    data: web::Json<Opinion>,
    jwt: JWTToken,
) -> APIResponse<Json<Opinion>> {
    validate_body(&data)?;
    let advert = data.create(&state.db, jwt.id).await?;
    respond_json(advert)
}

#[patch("/{id}")]
async fn delete_opinion(
    state: web::Data<AppState>,
    path: web::Path<Item>,
    jwt: JWTToken,
) -> APIResponse<HttpResponse> {
    Opinion::delete(&state.db, jwt.id, path.id).await?;
    respond_accepted()
}

#[get("/{id}")]
async fn get_all_opinions(
    state: web::Data<AppState>,
    path: web::Path<Item>,
) -> APIResponse<Json<Vec<Opinion>>> {
    let opinions = Opinion::get_all(&state.db, path.id).await?;
    respond_json(opinions)
}

// async fn update_opinion()
