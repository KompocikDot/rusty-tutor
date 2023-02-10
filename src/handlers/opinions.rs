use actix_web::{Scope, web::{Json, self}, post, patch};

use crate::{response::{APIResponse, respond_json}, models::opinions::Opinion, AppState, extractors::{jwt::JWTToken, path::Item}, validate::validate_body};

pub fn opinions_scope() -> Scope {
    Scope::new("/opinions")
    .service(create_opinion)
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
async fn delete_opinion(state: web::Data<AppState>, path: web::Path<Item>) -> APIResponse<Json<Opinion>> {
    todo!()
}