use uuid::Uuid;
use crate::modules::posts::domain::{entity::{Post, NewPost}, repository::PostRepository};
use crate::common::errors::AppError;

pub struct PostService<R: PostRepository> {
    repo: R,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_post(&self, title: String, content: String, author_id: Uuid) -> Result<Post, AppError> {
        let new_post = NewPost {
            title,
            content,
            author_id,
        };
        self.repo.create(new_post)
    }

    pub fn get_post(&self, id: Uuid) -> Result<Post, AppError> {
        self.repo.find_by_id(id)?
            .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))
    }

    pub fn list_posts(&self, page: i64, per_page: i64) -> Result<Vec<Post>, AppError> {
        let limit = if per_page > 0 { per_page } else { 10 };
        let offset = if page > 0 { (page - 1) * limit } else { 0 };
        self.repo.find_all(limit, offset)
    }

    pub fn update_post(&self, id: Uuid, title: String, content: String, is_published: bool, user_id: Uuid, is_admin: bool) -> Result<Post, AppError> {
        let post = self.get_post(id)?;
        
        if post.author_id != user_id && !is_admin {
            return Err(AppError::Forbidden("You do not have permission to update this post".to_string()));
        }

        self.repo.update(id, title, content, is_published)
    }

    pub fn delete_post(&self, id: Uuid, user_id: Uuid, is_admin: bool) -> Result<(), AppError> {
        let post = self.get_post(id)?;
        
        if post.author_id != user_id && !is_admin {
            return Err(AppError::Forbidden("You do not have permission to delete this post".to_string()));
        }

        self.repo.delete(id)
    }
}
