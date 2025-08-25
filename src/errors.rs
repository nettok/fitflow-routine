use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bb8::RunError;
use sentry::protocol::SpanId;
use serde::Serialize;
use std::panic::Location;
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
    #[error("Serialization/Deserialization error: {source}")]
    SerDeError {
        location: &'static Location<'static>,
        source: serde_json::Error,
    },

    #[error("Redis pool error: {source}")]
    RedisPoolError {
        location: &'static Location<'static>,
        source: RunError<redis::RedisError>,
    },

    #[error("Redis pool error: {source}")]
    RedisError {
        location: &'static Location<'static>,
        source: redis::RedisError,
    },
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

impl From<serde_json::Error> for AppError {
    #[track_caller]
    fn from(value: serde_json::Error) -> Self {
        AppError::SerDeError {
            location: Location::caller(),
            source: value,
        }
    }
}

impl From<RunError<redis::RedisError>> for AppError {
    #[track_caller]
    fn from(value: RunError<redis::RedisError>) -> Self {
        AppError::RedisPoolError {
            location: Location::caller(),
            source: value,
        }
    }
}

impl From<redis::RedisError> for AppError {
    #[track_caller]
    fn from(value: redis::RedisError) -> Self {
        AppError::RedisError {
            location: Location::caller(),
            source: value,
        }
    }
}
