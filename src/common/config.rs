use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiration_min: i64,
    pub jwt_refresh_expiration_days: i64,
    pub resend_api_key: String,
    pub app_url: String,
    pub email_from: String,
}


impl AppConfig {
    pub fn init() -> Self {
        dotenv().ok();

        let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid u16");
        
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        
        let jwt_access_expiration_min = env::var("JWT_ACCESS_EXPIRATION_MIN")
            .unwrap_or_else(|_| "15".to_string())
            .parse::<i64>()
            .expect("JWT_ACCESS_EXPIRATION_MIN must be a valid number");

        let jwt_refresh_expiration_days = env::var("JWT_REFRESH_EXPIRATION_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse::<i64>()
            .expect("JWT_REFRESH_EXPIRATION_DAYS must be a valid number");

        let resend_api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
        let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let email_from = env::var("EMAIL_FROM").unwrap_or_else(|_| "onboarding@resend.dev".to_string());

        Self {
            server_address,
            server_port,
            database_url,
            jwt_secret,
            jwt_access_expiration_min,
            jwt_refresh_expiration_days,
            resend_api_key,
            app_url,
            email_from,
        }

    }
}
