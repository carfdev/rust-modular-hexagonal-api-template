use uuid::Uuid;
use crate::common::errors::AppError;
use super::entity::{Post, NewPost};

pub trait PostRepository {
    fn create(&self, new_post: NewPost) -> Result<Post, AppError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError>;
    fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError>;
    fn update(&self, id: Uuid, title: String, content: String, is_published: bool) -> Result<Post, AppError>;
    fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

