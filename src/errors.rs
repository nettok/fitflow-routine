use axum::http::StatusCode;
use axum::response::{IntoResponse, NoContent, Response};
use bb8::RunError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Serialization/Deserialization error: {0}")]
    SerDeError(#[from] serde_json::Error),

    #[error("Redis pool error: {0}")]
    RedisPoolError(#[from] RunError<redis::RedisError>),

    #[error("Redis pool error: {0}")]
    RedisError(#[from] redis::RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, NoContent).into_response()
    }
}
