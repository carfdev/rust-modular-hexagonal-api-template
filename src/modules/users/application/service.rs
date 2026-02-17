use uuid::Uuid;
use crate::modules::users::domain::{entity::User, repository::UserRepository};
use crate::common::errors::AppError;


pub struct UserService<R: UserRepository> {
    user_repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub fn find_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        self.user_repo.find_by_id(id)?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
    }

    #[allow(dead_code)]
    pub fn find_user_by_email(&self, email: &str) -> Result<User, AppError> {
        self.user_repo.find_by_email(email)?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))
    }

    pub fn assign_role(&self, user_id: Uuid, role: &str) -> Result<(), AppError> {
        let _ = self.user_repo.find_by_id(user_id)?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;

        let roles = self.user_repo.get_roles(user_id)?;
        if roles.contains(&role.to_string()) {
            return Err(AppError::Conflict(format!("User already has role '{}'", role)));
        }

        self.user_repo.add_role(user_id, role)
    }

    pub fn remove_role(&self, user_id: Uuid, role: &str) -> Result<(), AppError> {

        let _ = self.user_repo.find_by_id(user_id)?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;

        let roles = self.user_repo.get_roles(user_id)?;
        if !roles.contains(&role.to_string()) {
            return Err(AppError::Conflict(format!("User does not have role '{}'", role)));
        }
        
        self.user_repo.remove_role(user_id, role)
    }
}
