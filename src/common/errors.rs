use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal Server Error")]
    InternalError,

    #[error("Database error")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Conflict: {0}")]
    Conflict(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,

            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        
        let details = match self {
            AppError::ValidationError(errors) => Some(json!(errors)),
            _ => None,
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            code: status_code.as_u16(),
            error: status_code.canonical_reason().unwrap_or("Unknown").to_string(),
            message: self.to_string(),
            details,
        })
    }
}
