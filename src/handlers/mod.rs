use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod webhook;

#[derive(Debug)]
pub enum HandlerError {
    ValidationError(String),
    AuthenticationError(String),
    InternalError(String),
    DatabaseError(String),
    NotificationError(String),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HandlerError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            HandlerError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            HandlerError::NotificationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            HandlerError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            HandlerError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;
