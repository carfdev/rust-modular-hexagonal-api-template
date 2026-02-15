use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::auth::domain::{entity::{UserSession, NewUserSession}, repository::SessionRepository};
use crate::schema::user_sessions;

pub struct DieselSessionRepository {
    pool: DbPool,
}

impl DieselSessionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl SessionRepository for DieselSessionRepository {
    fn create(&self, session: NewUserSession) -> Result<UserSession, AppError> {
        let mut conn = self.pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection: {}", e);
            AppError::InternalError
        })?;
        
        diesel::insert_into(user_sessions::table)
            .values(&session)
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<UserSession>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        user_sessions::table
            .find(id)
            .first::<UserSession>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn update_last_used(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection: {}", e);
            AppError::InternalError
        })?;
        
        diesel::update(user_sessions::table.find(id))
            .set(user_sessions::last_used_at.eq(diesel::dsl::now))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn update_refresh_token(&self, id: Uuid, new_hash: String, new_expires_at: chrono::NaiveDateTime) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection: {}", e);
            AppError::InternalError
        })?;
        
        diesel::update(user_sessions::table.find(id))
            .set((
                user_sessions::refresh_token_hash.eq(new_hash),
                user_sessions::expires_at.eq(new_expires_at),
                user_sessions::last_used_at.eq(diesel::dsl::now)
            ))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn revoke(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(user_sessions::table.find(id))
            .set(user_sessions::is_revoked.eq(true))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(user_sessions::table.filter(user_sessions::user_id.eq(user_id)))
            .set(user_sessions::is_revoked.eq(true))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn find_active_by_user(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        user_sessions::table
            .filter(user_sessions::user_id.eq(user_id))
            .filter(user_sessions::is_revoked.eq(false))
            .filter(user_sessions::expires_at.gt(diesel::dsl::now))
            .order(user_sessions::created_at.desc())
            .load::<UserSession>(&mut conn)
            .map_err(AppError::from)
    }
}
