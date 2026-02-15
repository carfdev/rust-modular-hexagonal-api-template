use actix_web::web;
use super::handlers::{create_post, get_post, list_posts, update_post, delete_post};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(list_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post))
    );
}
