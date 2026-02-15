use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::common::{errors::AppError, config::AppConfig};
use crate::modules::auth::domain::token::Claims;

pub struct TokenService {
    config: AppConfig,
}

impl TokenService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn generate_access_token(&self, user_id: Uuid, session_id: Uuid, roles: Vec<String>) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(self.config.jwt_access_expiration_min))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            session_id,
            roles,
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("Token generation failed: {}", e);
            AppError::InternalError
        })
    }

    pub fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }
}
