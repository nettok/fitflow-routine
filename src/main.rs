mod api;
mod config;

use crate::config::load_app_config;
use axum::response::Html;
use axum::routing::{get, put};
use axum::{BoxError, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use dotenvy::dotenv;
use redis::AsyncCommands;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Deserialize)]
struct AppConfig {
    redis_url: String,
}

#[derive(Clone)]
struct AppState {
    config: AppConfig,
    redis_pool: RedisPool,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();
    let config = load_app_config::<AppConfig>()?;
    let shared_config = config.clone();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        config,
        redis_pool: create_redis_pool(&shared_config.redis_url).await,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(state)).await.unwrap();
    Ok(())
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

type RedisPool = Pool<RedisConnectionManager>;

async fn create_redis_pool(redis_url: &str) -> RedisPool {
    tracing::info!("connecting to redis");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    {
        // ping the database before starting
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        assert_eq!(result, "bar");
        conn.del::<&str, usize>("foo").await.unwrap();
    }
    tracing::info!("successfully connected to redis and pinged it");
    pool
}
