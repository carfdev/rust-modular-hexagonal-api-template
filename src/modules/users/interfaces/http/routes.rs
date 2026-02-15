use actix_web::web;
use super::handlers::{get_me, assign_role, remove_role};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/{id}/roles", web::post().to(assign_role))
            .route("/{id}/roles/{role}", web::delete().to(remove_role))
    );
}
