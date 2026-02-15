use uuid::Uuid;
use super::entity::{UserSession, NewUserSession};
use crate::common::errors::AppError;

pub trait SessionRepository {

    fn create(&self, session: NewUserSession) -> Result<UserSession, AppError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<UserSession>, AppError>;
    fn update_last_used(&self, id: Uuid) -> Result<(), AppError>;
    fn update_refresh_token(&self, id: Uuid, new_hash: String, new_expires_at: chrono::NaiveDateTime) -> Result<(), AppError>;
    fn revoke(&self, id: Uuid) -> Result<(), AppError>;
    fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), AppError>;
    fn find_active_by_user(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError>;
}
pub mod verification;
