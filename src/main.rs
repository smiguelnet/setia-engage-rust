use std::env;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

mod controller;
mod models;
mod persistence;

use deadpool_postgres::{Config, PoolConfig, Runtime};
use models::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Setting up app from environment");
    let db_user = env::var("DB_USER").unwrap_or("postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or("postgres".to_string());
    let db_name = env::var("DB_NAME").unwrap_or("engagedb".to_string());
    let db_port_param = env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_port = db_port_param.parse::<u16>().unwrap();
    let db_pool_size_param = env::var("DB_POOL_SIZE").unwrap_or("100".to_string());
    let db_pool_size = db_pool_size_param.parse::<usize>().unwrap();

    log::info!("Setting up db and pool configuration");
    let mut db_pool_config = Config::new();
    db_pool_config.host = Some(env::var("DB_HOST").unwrap_or("localhost".to_string()));
    db_pool_config.port = Some(db_port);
    db_pool_config.dbname = Some(db_name);
    db_pool_config.user = Some(db_user);
    db_pool_config.password = Some(db_password);
    db_pool_config.pool = PoolConfig::new(db_pool_size).into();

    log::info!("Creating db pool...");
    let db_pool = db_pool_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)?;
    log::info!("db pool created successfully");

    let http_port: String = env::var("HTTP_PORT").unwrap_or("8080".into());
    let http_port_cors = http_port.clone();
    log::info!(
        "Starting HTTP server. Port {} / Cors port: {}",
        http_port,
        http_port_cors
    );

    let app_state = AppState {
        app_name: String::from("Engage API"),
        app_version: String::from("0.1.1"),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(format!("http://localhost:{}", http_port_cors).as_str())
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .service(controller::app)
            .service(controller::get_users)
            .service(controller::health_check)
    })
    .bind(format!("0.0.0.0:{}", http_port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use reqwest::StatusCode;

    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let app = App::new().service(controller::health_check);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert!(body.is_empty());
    }
}
