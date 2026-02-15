use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::posts::domain::{
    entity::{Post, NewPost},
    repository::PostRepository,
};
use crate::schema::posts;

pub struct DieselPostRepository {
    pool: DbPool,
}

impl DieselPostRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PostRepository for DieselPostRepository {
    fn create(&self, new_post: NewPost) -> Result<Post, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        posts::table
            .find(id)
            .first::<Post>(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        posts::table
            .limit(limit)
            .offset(offset)
            .order(posts::created_at.desc())
            .load::<Post>(&mut conn)
            .map_err(AppError::from)
    }

    fn update(&self, id: Uuid, title: String, content: String, is_published: bool) -> Result<Post, AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::update(posts::table.find(id))
            .set((
                posts::title.eq(title),
                posts::content.eq(content),
                posts::is_published.eq(is_published),
                posts::updated_at.eq(diesel::dsl::now),
            ))
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|_| AppError::InternalError)?;
        
        diesel::delete(posts::table.find(id))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(AppError::from)
    }
}

