use crate::models::*;

use actix_web::{get, post, web, HttpResponse, Responder, Result};

#[get("/")]
pub async fn app(app_state: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("Main route");
    let body = serde_json::to_string(&app_state);
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(body.unwrap()))
}

#[post("/user")]
pub async fn get_user(user: web::Json<User>) -> web::Json<User> {
    log::info!("User route / get_user");
    web::Json(User {
        id: Some(uuid::Uuid::new_v4().to_string()),
        name: user.name.clone(),
        age: user.age.clone(),
    })
}
