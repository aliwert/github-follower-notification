pub mod webhook;


#[derive(Debug)]
pub enum HandlerError {
    ValidationError(String),
    AuthenticationError(String),
    InternalError(String),
    DatabaseError(String),
    NotificationError(String),
}

pub type HandlerResult<T> = Result<T, HandlerError>;