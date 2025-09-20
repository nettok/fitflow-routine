mod api;
mod config;
mod errors;
mod events;
mod metrics;
mod redis_pool;
mod sentry_tracing;

use crate::config::{RunProfile, load_app_config};
use crate::metrics::metrics_app;
use crate::redis_pool::RedisPool;
use crate::sentry_tracing::init_tracing_with_sentry;
use axum::body::Body;
use axum::http::Request;
use axum::response::Html;
use axum::routing::{get, put};
use axum::{BoxError, Router, middleware};
use dotenvy::dotenv;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct AppConfig {
    run_profile: RunProfile,
    sentry_dsn: Option<String>,
    redis_url: String,
}

#[derive(Clone)]
struct AppState {
    config: AppConfig,
    redis_pool: RedisPool,
}

fn main() -> Result<(), BoxError> {
    dotenv().ok();
    let config = load_app_config::<AppConfig>()?;
    let shared_config = config.clone();

    init_tracing_with_sentry();

    let _guard = if let Some(sentry_dsn) = shared_config.sentry_dsn {
        Some(sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                enable_logs: true,
                traces_sample_rate: 1.0f32,
                send_default_pii: true,
                environment: Some(shared_config.run_profile.to_string().into()),
                ..Default::default()
            },
        )))
    } else {
        None
    };

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let state = AppState {
                config,
                redis_pool: redis_pool::new_redis_pool(&shared_config.redis_url)
                    .await
                    .unwrap(),
            };

            events::handle_events(state.redis_pool.clone());

            tokio::join!(main_server(state), metrics_server())
        });
    Ok(())
}

async fn main_server(state: AppState) {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!(
        "main server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, main_app(state)).await.unwrap();
}

async fn metrics_server() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    tracing::info!(
        "metrics server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, metrics_app()).await.unwrap();
}

fn main_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/healthz", get(api::health::get_healthz))
        .nest(
            "/api/v1",
            Router::new()
                .route(
                    "/routines/by-goal",
                    get(api::routines::get_routines_by_goal),
                )
                .route("/routines/goals", get(api::routines::get_goals))
                .route(
                    "/assignments/{user_id}",
                    get(api::assignments::get_user_assignments),
                )
                .route(
                    "/assignments/{user_id}/accept/{routine_id}",
                    put(api::assignments::assignment_accept),
                )
                .route(
                    "/assignments/{user_id}/start/{routine_id}",
                    put(api::assignments::assignment_start),
                )
                .route(
                    "/assignments/{user_id}/complete/{routine_id}",
                    put(api::assignments::assignment_complete),
                ),
        )
        .with_state(state)
        .route_layer(middleware::from_fn(metrics::track_metrics))
        .layer(NewSentryLayer::<Request<Body>>::new_from_top())
        .layer(SentryHttpLayer::new().enable_transaction())
}

#[tracing::instrument]
async fn handle_index() -> Html<&'static str> {
    Html("OK")
}
