use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    Internal(String),
    Database(sqlx::Error),
    Nats(async_nats::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::Database(e) => write!(f, "Database error: {}", e),
            ApiError::Nats(e) => write!(f, "NATS error: {}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg.clone()),
            ApiError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
            ApiError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "A database error occurred".to_string(),
                )
            }
            ApiError::Nats(e) => {
                tracing::error!("NATS error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "queue_error",
                    "A queue error occurred".to_string(),
                )
            }
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message,
        })
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Database(e)
    }
}

impl From<async_nats::Error> for ApiError {
    fn from(e: async_nats::Error) -> Self {
        ApiError::Nats(e)
    }
}
