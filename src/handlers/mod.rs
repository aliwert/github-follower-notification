pub mod webhook;
pub mod handlers;
pub mod models;
pub mod services;
pub mod config;

pub use handlers::{HandlerError, HandlerResult};
pub use models::events::FollowerEvent;
pub use services::NotificationService;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum HandlerError {
    ValidationError(String),
    AuthenticationError(String),
    InternalError(String),
    DatabaseError(String),
    NotificationError(String),
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::NotificationError(msg) => write!(f, "Notification error: {}", msg),
        }
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            Self::ValidationError(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
            Self::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, "AUTH_ERROR", msg),
            Self::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg),
            Self::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", msg),
            Self::NotificationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "NOTIFICATION_ERROR", msg),
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "type": error_type,
            "message": message
        }));

        (status, body).into_response()
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;

impl From<anyhow::Error> for HandlerError {
    fn from(err: anyhow::Error) -> Self {
        HandlerError::InternalError(err.to_string())
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(err: sqlx::Error) -> Self {
        HandlerError::DatabaseError(err.to_string())
    }
}