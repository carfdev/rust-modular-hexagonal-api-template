use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::users::domain::{entity::{User, NewUser}, repository::UserRepository};
use crate::schema::users;

pub struct DieselUserRepository {
    pool: DbPool,
}

impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for DieselUserRepository {
    fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection: {}", e);
            AppError::InternalError
        })?;
        
        users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }
    fn find_by_username(&self, username_val: &str) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        users::table
            .filter(users::username.eq(username_val))
            .first::<User>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        users::table
            .find(id)
            .first::<User>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn verify_user(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;

        diesel::update(users::table.find(id))
            .set(users::is_verified.eq(true))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn get_roles(&self, user_id_val: Uuid) -> Result<Vec<String>, AppError> {
        use crate::schema::{roles, user_roles};
        
        let mut conn = self.pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection in get_roles: {}", e);
            AppError::InternalError
        })?;
        
        user_roles::table
            .filter(user_roles::user_id.eq(user_id_val))
            .inner_join(roles::table)
            .select(roles::name)
            .load::<String>(&mut conn)
            .map_err(|e| {
                tracing::error!("Failed to load roles: {}", e);
                AppError::from(e)
            })
    }

    fn add_role(&self, user_id_val: Uuid, role_name: &str) -> Result<(), AppError> {
        use crate::schema::{roles, user_roles};
        
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        // 1. Find role id
        let role_id_val: i32 = roles::table
            .filter(roles::name.eq(role_name))
            .select(roles::id)
            .first(&mut conn)
            .map_err(|_| AppError::NotFound(format!("Role {} not found", role_name)))?;
            
        // 2. Insert into user_roles
        diesel::insert_into(user_roles::table)
            .values((
                user_roles::user_id.eq(user_id_val),
                user_roles::role_id.eq(role_id_val)
            ))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn remove_role(&self, user_id_val: Uuid, role_name: &str) -> Result<(), AppError> {
        use crate::schema::{roles, user_roles};
        
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        // 1. Find role id
        let role_id_val: i32 = roles::table
            .filter(roles::name.eq(role_name))
            .select(roles::id)
            .first(&mut conn)
            .map_err(|_| AppError::NotFound(format!("Role {} not found", role_name)))?;
            
        // 2. Delete from user_roles
        diesel::delete(user_roles::table)
            .filter(user_roles::user_id.eq(user_id_val))
            .filter(user_roles::role_id.eq(role_id_val))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }

    fn update_password(&self, user_id: Uuid, new_password_hash: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(users::table.find(user_id))
            .set(users::password_hash.eq(new_password_hash))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }
}



