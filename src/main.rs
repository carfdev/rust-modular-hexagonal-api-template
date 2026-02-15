mod modules;
mod common;
mod schema;

use actix_web::{web, App, HttpServer, middleware::Logger};
use common::{config::AppConfig, logging, database::DbPool};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::init();
    
    let config = AppConfig::init();
    let app_config = config.clone();
    let pool = common::database::init(&config.database_url);
    let config_pool = pool.clone();

    let server_addr = format!("{}:{}", config.server_address, config.server_port);

    tracing::info!("Starting server at http://{}", server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DbPool::clone(&config_pool))) // Need to create pool outside
            .app_data(web::Data::new(app_config.clone()))
            .wrap(Logger::default())
            .configure(modules::auth::interfaces::http::routes::config)
            .configure(modules::users::interfaces::http::routes::config)
            .configure(modules::posts::interfaces::http::routes::config)


            .route("/", web::get().to(|| async { "Hello from Rust Hexagonal API!" }))
    })

    .bind(server_addr)?
    .run()
    .await
}
