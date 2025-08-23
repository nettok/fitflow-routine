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
