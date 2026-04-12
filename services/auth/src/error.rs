use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials(String),
    InvalidToken(String),
    ExpiredToken,
    JwtError(String),
    OAuthError(String),
    DatabaseError(sqlx::Error),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            AuthError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AuthError::ExpiredToken => write!(f, "Token expired"),
            AuthError::JwtError(msg) => write!(f, "JWT error: {}", msg),
            AuthError::OAuthError(msg) => write!(f, "OAuth error: {}", msg),
            AuthError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AuthError::RedisError(e) => write!(f, "Redis error: {}", e),
            AuthError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message) = match self {
            AuthError::InvalidCredentials(msg) => {
                (StatusCode::UNAUTHORIZED, "invalid_credentials", msg.clone())
            }
            AuthError::InvalidToken(msg) => {
                (StatusCode::UNAUTHORIZED, "invalid_token", msg.clone())
            }
            AuthError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, "expired_token", "Token has expired".to_string())
            }
            AuthError::JwtError(_) | AuthError::OAuthError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "auth_error", "Authentication error".to_string())
            }
            AuthError::DatabaseError(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "database_error", "Database error".to_string())
            }
            AuthError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal error".to_string())
            }
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message,
        })
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        AuthError::DatabaseError(e)
    }
}
