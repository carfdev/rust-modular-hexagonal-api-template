use uuid::Uuid;
use super::entity::{User, NewUser};
use crate::common::errors::AppError;

pub trait UserRepository {
    fn create(&self, new_user: NewUser) -> Result<User, AppError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    fn verify_user(&self, id: Uuid) -> Result<(), AppError>;
    fn get_roles(&self, user_id: Uuid) -> Result<Vec<String>, AppError>;
    fn add_role(&self, user_id: Uuid, role_name: &str) -> Result<(), AppError>;
    fn remove_role(&self, user_id: Uuid, role_name: &str) -> Result<(), AppError>;
    fn update_password(&self, user_id: Uuid, new_password_hash: &str) -> Result<(), AppError>;
    fn update_last_login(&self, user_id: Uuid) -> Result<(), AppError>;
}



