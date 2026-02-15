use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::modules::posts::domain::entity::Post;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostDto {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub is_published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub author_id: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            is_published: post.is_published,
            author_id: post.author_id,
            created_at: post.created_at.to_string(),
            updated_at: post.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationDto {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
