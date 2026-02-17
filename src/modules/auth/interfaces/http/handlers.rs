use actix_web::{web, HttpResponse};
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::auth::application::service::AuthService;
use crate::modules::users::infrastructure::diesel_repository::DieselUserRepository;
use crate::modules::auth::infrastructure::{
    diesel_repository::DieselSessionRepository,
    diesel_token_repository::DieselVerificationTokenRepository,
};
use crate::modules::email::infrastructure::resend::ResendEmailService;
use crate::common::config::AppConfig;
use super::dto::{RegisterUserDto, LoginDto, VerifyEmailDto};
use validator::Validate;

// Type alias with ALL generics
type AuthServiceImpl = AuthService<
    DieselUserRepository,
    DieselSessionRepository,
    DieselVerificationTokenRepository,
    ResendEmailService
>;

// Helper to create service
fn auth_service_factory(pool: &DbPool, config: &AppConfig) -> AuthServiceImpl {
    let user_repo = DieselUserRepository::new(pool.clone());
    let session_repo = DieselSessionRepository::new(pool.clone());
    let token_repo = DieselVerificationTokenRepository::new(pool.clone());
    let email_service = ResendEmailService::new(config.clone());
    let token_service = crate::modules::auth::application::token_service::TokenService::new(config.clone());
    
    AuthService::new(
        user_repo,
        session_repo,
        token_repo,
        email_service,
        token_service,
        config.clone()
    )
}

pub async fn register(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;
    
    let service = auth_service_factory(&pool, &config);
    service.register(body.email.clone(), body.password.clone()).await?;

    
    Ok(HttpResponse::Created().json(serde_json::json!({"message": "User registered successfully, please verify your email"})))
}

pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    req: actix_web::HttpRequest,
    body: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
        
    let ip_address = req.peer_addr().map(|a| a.ip().to_string());
    
    let service = auth_service_factory(&pool, &config);
    let (access_token, refresh_token) = service.login(body.email.clone(), body.password.clone(), user_agent, ip_address).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token
    })))
}

pub async fn verify_email(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<VerifyEmailDto>,
) -> Result<HttpResponse, AppError> {
    
    let service = auth_service_factory(&pool, &config);
    service.verify_email(body.token.clone()).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Email verified successfully"})))
}

#[derive(serde::Deserialize)]
pub struct RequestResetDto {
    pub email: String,
}

pub async fn request_password_reset(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<RequestResetDto>,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    service.request_password_reset(&body.email).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Password reset email sent"})))
}

pub async fn reset_password(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<super::dto::ResetPasswordDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;

    let service = auth_service_factory(&pool, &config);
    service.reset_password(&body.token, &body.new_password).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Password reset successfully"})))
}

use super::dto::RequestEmailVerificationDto;

pub async fn request_email_verification(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<RequestEmailVerificationDto>,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    service.request_email_verification(&body.email).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Verification email sent"})))
}

use super::middleware::AuthenticatedUser;

pub async fn logout(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    service.logout(user.session_id)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Logged out successfully"})))
}

pub async fn revoke_all_sessions(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    service.revoke_all_sessions(user.user_id)?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "All sessions revoked"})))
}

use super::dto::RefreshTokenDto;

pub async fn refresh_token(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    body: web::Json<RefreshTokenDto>,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    let (access_token, refresh_token) = service.refresh_token(&body.refresh_token).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token
    })))
}

use super::dto::UserSessionDto;

pub async fn get_active_sessions(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let service = auth_service_factory(&pool, &config);
    let sessions = service.get_active_sessions(user.user_id)?;
    
    let dtos: Vec<UserSessionDto> = sessions.into_iter().map(|s| UserSessionDto {
        id: s.id,
        user_agent: s.user_agent,
        ip_address: s.ip_address,
        device_name: s.device_name,
        is_revoked: s.is_revoked,
        created_at: s.created_at,
        last_used_at: s.last_used_at,
        expires_at: s.expires_at,
        is_current: s.id == user.session_id,
    }).collect();

    Ok(HttpResponse::Ok().json(dtos))
}