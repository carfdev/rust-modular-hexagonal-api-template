use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;
use crate::common::{database::DbPool, errors::AppError};
use crate::modules::posts::{
    application::service::PostService,
    infrastructure::diesel_repository::DieselPostRepository,
};
use crate::modules::auth::interfaces::http::middleware::AuthenticatedUser;
use super::dto::{CreatePostDto, UpdatePostDto, PostDto, PaginationDto};

type PostServiceImpl = PostService<DieselPostRepository>;

pub fn post_service_factory(pool: &DbPool) -> PostServiceImpl {
    let repo = DieselPostRepository::new(pool.clone());
    PostService::new(repo)
}

pub async fn create_post(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    body: web::Json<CreatePostDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;
    
    let service = post_service_factory(&pool);
    let post = service.create_post(body.title.clone(), body.content.clone(), user.user_id)?;
    
    Ok(HttpResponse::Created().json(PostDto::from(post)))
}

pub async fn get_post(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let service = post_service_factory(&pool);
    let post = service.get_post(path.into_inner())?;
    
    Ok(HttpResponse::Ok().json(PostDto::from(post)))
}

pub async fn list_posts(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationDto>,
) -> Result<HttpResponse, AppError> {
    let service = post_service_factory(&pool);
    let posts = service.list_posts(query.page.unwrap_or(1), query.per_page.unwrap_or(10))?;
    
    let dtos: Vec<PostDto> = posts.into_iter().map(PostDto::from).collect();
    Ok(HttpResponse::Ok().json(dtos))
}

pub async fn update_post(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePostDto>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(AppError::ValidationError)?;
    let post_id = path.into_inner();
    let service = post_service_factory(&pool);
    
    let is_admin = user.roles.iter().any(|r| r == "admin");
    
    let post = service.update_post(
        post_id, 
        body.title.clone(), 
        body.content.clone(), 
        body.is_published, 
        user.user_id, 
        is_admin
    )?;
    
    Ok(HttpResponse::Ok().json(PostDto::from(post)))
}

pub async fn delete_post(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let service = post_service_factory(&pool);
    
    let is_admin = user.roles.iter().any(|r| r == "admin");
    
    service.delete_post(post_id, user.user_id, is_admin)?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Post deleted successfully"})))
}
