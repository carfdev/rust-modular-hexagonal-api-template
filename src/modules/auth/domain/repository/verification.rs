
use uuid::Uuid;
use crate::modules::auth::domain::entity::token::{EmailVerificationToken, NewEmailVerificationToken, PasswordResetToken, NewPasswordResetToken};

use crate::common::errors::AppError;

pub trait VerificationTokenRepository {

    fn create_email_verification(&self, token: NewEmailVerificationToken) -> Result<EmailVerificationToken, AppError>;
    fn find_email_verification_by_user(&self, user_id: Uuid) -> Result<Option<EmailVerificationToken>, AppError>;
    // In real app we might search by token hash, but we only have hash. Logic: Find by user, check all valid tokens?
    // Or normally we pass token ID + Token string.
    // If we only send Token String (random), we need to look up which user it belongs to?
    // Usually the link is /verify?token=XYZ&id=USER_ID.
    // So we find by UserID, then verify hash.
    
    fn create_password_reset(&self, token: NewPasswordResetToken) -> Result<PasswordResetToken, AppError>;
    fn find_password_reset_by_user(&self, user_id: Uuid) -> Result<Option<PasswordResetToken>, AppError>;
    
    fn mark_email_verification_as_used(&self, token_id: Uuid) -> Result<(), AppError>;
    fn mark_password_reset_as_used(&self, token_id: Uuid) -> Result<(), AppError>;
}
