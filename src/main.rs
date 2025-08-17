mod api;
mod config;
mod events;
mod redis_pool;

use crate::config::{load_app_config, RunProfile};
use crate::redis_pool::RedisPool;
use axum::response::Html;
use axum::routing::{get, put};
use axum::{BoxError, Router};
use dotenvy::dotenv;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Deserialize)]
struct AppConfig {
    run_profile: RunProfile,
    sentry_dns: String,
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

    let _guard = sentry::init((shared_config.sentry_dns, sentry::ClientOptions {
        release: sentry::release_name!(),
        send_default_pii: true,
        environment: Some(shared_config.run_profile.to_string().into()),
        ..Default::default()
    }));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            init_tracing();

            let state = AppState {
                config,
                redis_pool: redis_pool::new_redis_pool(&shared_config.redis_url).await,
            };

            events::handle_events(state.redis_pool.clone());

            let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

            println!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app(state)).await.unwrap();
        });
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(handle_index))
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
}

async fn handle_index() -> Html<&'static str> {
    Html("OK")
}
