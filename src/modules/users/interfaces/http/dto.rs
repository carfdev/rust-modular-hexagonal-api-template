use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::modules::users::domain::entity::User;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub is_verified: bool,
    pub created_at: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            is_verified: user.is_verified,
            created_at: user.created_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssignRoleDto {
    #[validate(length(min = 1))]
    pub role: String,
}
