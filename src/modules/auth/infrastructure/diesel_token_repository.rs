use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::auth::domain::{
    entity::token::{EmailVerificationToken, NewEmailVerificationToken, PasswordResetToken, NewPasswordResetToken},
    repository::verification::VerificationTokenRepository,
};
use crate::schema::{email_verification_tokens, password_reset_tokens};

pub struct DieselVerificationTokenRepository {
    pool: DbPool,
}

impl DieselVerificationTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl VerificationTokenRepository for DieselVerificationTokenRepository {
    fn create_email_verification(&self, token: NewEmailVerificationToken) -> Result<EmailVerificationToken, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::insert_into(email_verification_tokens::table)
            .values(&token)
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn find_email_verification_by_user(&self, user_id_val: Uuid) -> Result<Option<EmailVerificationToken>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        email_verification_tokens::table
            .filter(email_verification_tokens::user_id.eq(user_id_val))
            .filter(email_verification_tokens::used.eq(false)) // Only unused
            .order(email_verification_tokens::created_at.desc())
            .first::<EmailVerificationToken>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn create_password_reset(&self, token: NewPasswordResetToken) -> Result<PasswordResetToken, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::insert_into(password_reset_tokens::table)
            .values(&token)
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn find_password_reset_by_user(&self, user_id_val: Uuid) -> Result<Option<PasswordResetToken>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        password_reset_tokens::table
            .filter(password_reset_tokens::user_id.eq(user_id_val))
            .filter(password_reset_tokens::used.eq(false))
            .order(password_reset_tokens::created_at.desc())
            .first::<PasswordResetToken>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn mark_email_verification_as_used(&self, token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(email_verification_tokens::table.find(token_id))
            .set(email_verification_tokens::used.eq(true))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn mark_password_reset_as_used(&self, token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(password_reset_tokens::table.find(token_id))
            .set(password_reset_tokens::used.eq(true))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }
}
