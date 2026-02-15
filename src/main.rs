mod modules;
mod common;
mod schema;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http::header};
use common::{config::AppConfig, logging, database::DbPool};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env/config first so RUST_LOG is available
    let config = AppConfig::init();
    
    // Init logging after env is loaded
    logging::init();
    
    let app_config = config.clone();
    let pool = common::database::init(&config.database_url);
    let config_pool = pool.clone();

    let server_addr = format!("{}:{}", config.server_address, config.server_port);

    tracing::info!("Starting server at http://{}", server_addr);

    let allowed_origin = config.app_url.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(DbPool::clone(&config_pool))) // Need to create pool outside
            .app_data(web::Data::new(app_config.clone()))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Use standard Logger for visible request logs
            .configure(modules::auth::interfaces::http::routes::config)
            .configure(modules::users::interfaces::http::routes::config)
            .configure(modules::posts::interfaces::http::routes::config)


            .route("/", web::get().to(|| async { "Hello from Rust Hexagonal API!" }))
    })

    .bind(server_addr)?
    .run()
    .await
}
