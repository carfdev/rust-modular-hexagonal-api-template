use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailDto {
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordDto {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestEmailVerificationDto {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UserSessionDto {
    pub id: uuid::Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub device_name: Option<String>,
    pub is_revoked: bool,
    pub created_at: chrono::NaiveDateTime,
    pub last_used_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub is_current: bool,
}
