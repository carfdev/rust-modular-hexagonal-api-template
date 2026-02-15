use async_trait::async_trait;
use crate::common::errors::AppError;

#[derive(Debug, Clone)]
pub struct EmailRecipient {
    pub email: String,
    pub name: Option<String>,


}

#[async_trait]
pub trait EmailService: Send + Sync {

    async fn send_verification_email(&self, recipient: &EmailRecipient, token: &str) -> Result<(), AppError>;
    async fn send_password_reset_email(&self, recipient: &EmailRecipient, token: &str) -> Result<(), AppError>;
}
