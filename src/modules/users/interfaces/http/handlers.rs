use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::users::{
    application::service::UserService,
    infrastructure::diesel_repository::DieselUserRepository,
};
use crate::modules::auth::interfaces::http::middleware::{AuthenticatedUser, RequireAdmin};
use super::dto::{UserDto, AssignRoleDto};

// Type alias
type UserServiceImpl = UserService<DieselUserRepository>;

pub fn user_service_factory(pool: &DbPool) -> UserServiceImpl {
    let user_repo = DieselUserRepository::new(pool.clone());
    UserService::new(user_repo)
}

pub async fn get_me(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let service = user_service_factory(&pool);
    let user_entity = service.find_user_by_id(user.user_id)?;
    Ok(HttpResponse::Ok().json(UserDto::from(user_entity)))
}

pub async fn assign_role(
    _admin: RequireAdmin, // Guard
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    body: web::Json<AssignRoleDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;
    let user_id = path.into_inner();
    
    let service = user_service_factory(&pool);
    service.assign_role(user_id, &body.role)?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Role assigned successfully"})))
}

pub async fn remove_role(
    _admin: RequireAdmin,
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, String)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, role) = path.into_inner();
    
    let service = user_service_factory(&pool);
    service.remove_role(user_id, &role)?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Role removed successfully"})))
}
