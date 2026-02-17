use actix_web::{FromRequest, dev::Payload, web, HttpRequest};
use crate::common::errors::AppError;




use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::modules::auth::application::token_service::TokenService;

use crate::common::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub roles: Vec<String>,
}

use std::pin::Pin;
use std::future::Future;
use crate::common::database::DbPool;
use crate::modules::auth::domain::repository::SessionRepository;
use crate::modules::auth::infrastructure::diesel_repository::DieselSessionRepository;

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization").map(|h| h.clone());
        let config = req.app_data::<web::Data<AppConfig>>().cloned();
        let pool = req.app_data::<web::Data<DbPool>>().cloned();

        Box::pin(async move {
            let auth_str = match auth_header {
                Some(h) => h.to_str().map_err(|_| AppError::Unauthorized("Invalid authorization header".to_string()))?.to_string(),
                None => return Err(AppError::Unauthorized("Missing authorization header".to_string()).into()),
            };

            if !auth_str.starts_with("Bearer ") {
                 return Err(AppError::Unauthorized("Invalid token scheme".to_string()).into());
            }
            let token = auth_str[7..].to_string();

            let config = config.ok_or_else(|| AppError::InternalError)?;
            let pool = pool.ok_or_else(|| AppError::InternalError)?;
            
            let token_service = TokenService::new(config.as_ref().clone());

            let claims = token_service.verify_access_token(&token)
                .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;
                
            // Validate session in DB
            let session_repo = DieselSessionRepository::new(pool.as_ref().clone());
            let session_id = claims.session_id;

            // Blocking call wrapper
            let session = web::block(move || session_repo.find_by_id(session_id))
                .await
                .map_err(|_| AppError::InternalError)?
                .map_err(|_| AppError::InternalError)?;

            match session {
                Some(s) if !s.is_revoked => {
                    // Update last used (fire and forget or await)
                    // We need a new repo instance or clone because `find_by_id` moved it? 
                    // No, `find_by_id` takes &self. But `web::block` move closure?
                    // Let's create repo inside block.
                    
                    // Actually, let's do update in another block or same if possible.
                    // But `find_by_id` returns `UserSession` which we need to check.
                    
                    // Update last_used
                     let _ = web::block(move || {
                        let repo = DieselSessionRepository::new(pool.as_ref().clone());
                        repo.update_last_used(session_id)
                     }).await; // Ignore error on update stats
                     
                    Ok(AuthenticatedUser {
                        user_id: claims.sub,
                        session_id: claims.session_id,
                        roles: claims.roles,
                    })
                },
                _ => Err(AppError::Unauthorized("Session invalid or expired".to_string()).into()),
            }
        })
    }
}


pub struct RequireAdmin;

impl FromRequest for RequireAdmin {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let auth_future = AuthenticatedUser::from_request(req, payload);
        
        Box::pin(async move {
            let user = auth_future.await?;
            
            let has_role = user.roles.iter().any(|r| r == "admin");
            if has_role {
                Ok(RequireAdmin)
            } else {
                Err(AppError::Forbidden("Admin role required".to_string()).into())
            }
        })
    }
}


