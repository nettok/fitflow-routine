use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bb8::RunError;
use sentry::protocol::SpanId;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: ErrorData,
}

#[derive(Debug, Serialize)]
struct ErrorData {
    debug_id: Option<SpanId>,
}

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

        let span_id = sentry::Hub::current()
            .configure_scope(|scope| scope.get_span())
            .map(|span| span.get_trace_context().span_id);

        let error_response = ErrorResponse {
            error: ErrorData { debug_id: span_id },
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
    }
}
