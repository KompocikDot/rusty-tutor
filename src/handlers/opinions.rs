use actix_web::{Scope, web::{Json, self}, post, patch, HttpResponse, get};

use crate::{response::{respond_json, respond_accepted}, models::opinions::Opinion, AppState, extractors::{jwt::JWTToken, path::Item}, validate::validate_body, types::APIResponse};

pub fn opinions_scope() -> Scope {
    Scope::new("/opinions")
    .service(create_opinion)
    .service(get_all_opinions)
}


#[post("/")]
async fn create_opinion(
    state: web::Data<AppState>,
    data: web::Json<Opinion>,
    jwt: JWTToken
) -> APIResponse<Json<Opinion>> {
    validate_body(&data)?;
    let advert = data.create(&state.db, jwt.id).await?;
    respond_json(advert)
}

#[patch("/{id}")]
async fn delete_opinion(state: web::Data<AppState>, path: web::Path<Item>, jwt: JWTToken) -> APIResponse<HttpResponse> {
    Opinion::delete(&state.db, jwt.id, path.id).await?;
    respond_accepted()
}

#[get("/{id}")]
async fn get_all_opinions(state: web::Data<AppState>, path: web::Path<Item>) -> APIResponse<Json<Vec<Opinion>>> {
    let opinions = Opinion::get_all(&state.db, path.id).await?;
    respond_json(opinions)
}

// async fn update_opinion()