use crate::models::*;
use crate::persistence::*;

use actix_web::{web, HttpResponse, Responder, Result};
use deadpool_postgres::Client;
use deadpool_postgres::Pool;

pub type APIResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[actix_web::get("/")]
pub async fn app(app_state: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("Main route");
    let body = serde_json::to_string(&app_state);
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(body.unwrap()))
}

#[actix_web::get("/users")]
pub async fn get_users(db_pool: web::Data<Pool>) -> APIResult {
    log::info!("Ger users");
    let client: Client = db_pool.get().await?;
    let users = db_list_users(&client).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[actix_web::get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
