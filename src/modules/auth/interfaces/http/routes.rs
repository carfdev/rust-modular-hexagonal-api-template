use actix_web::web;
use web::{post, get};
use super::handlers::{register, login, verify_email, request_email_verification, request_password_reset, reset_password, logout, revoke_all_sessions, refresh_token, get_active_sessions};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
             .route("/register", post().to(register))
             .route("/login", post().to(login))
             .route("/refresh", post().to(refresh_token))
             .route("/verify-email", post().to(verify_email))
             .route("/request-email-verification", post().to(request_email_verification))
             .route("/request-password-reset", post().to(request_password_reset))
             .route("/reset-password", post().to(reset_password))
             .route("/logout", post().to(logout))
             .route("/sessions", get().to(get_active_sessions))
             .route("/sessions/revoke-all", post().to(revoke_all_sessions))
    );
}
