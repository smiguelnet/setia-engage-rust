use std::env;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, Error, HttpServer};

mod controller;
mod models;

use models::AppState;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

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
            .service(controller::app)
            .service(controller::get_user)
    })
    .bind(format!("0.0.0.0:{}", http_port))?
    .run()
    .await?;

    Ok(())
}
