use crate::errors::AppError;
use crate::redis_pool::RedisPool;
use redis::AsyncTypedCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

#[derive(Deserialize, Debug)]
struct GoalSetEvent {
    user_id: String,
    goal: String,
}

#[derive(Serialize, Debug)]
struct RoutineRecommendedEvent {
    user_id: String,
    routine_id: String,
}

pub fn handle_events(redis_pool: RedisPool) {
    tokio::spawn(async move {
        let backoff_delay = Duration::from_secs(5);
        let mut restart_count = 0;

        loop {
            tracing::info!(
                "Starting event processing loop (restart count: {})",
                restart_count
            );

            let result = tokio::spawn(event_processing_loop(redis_pool.clone())).await;

            match result {
                Ok(()) => {
                    tracing::warn!("Event processing loop completed normally, restarting...");
                }
                Err(join_error) => {
                    if join_error.is_panic() {
                        tracing::error!("Event processing loop panicked: {:?}", join_error);
                    } else {
                        tracing::error!("Event processing loop was cancelled: {:?}", join_error);
                    }
                }
            }

            restart_count += 1;
            tracing::info!("Restarting event processing in {:?}", backoff_delay);
            time::sleep(backoff_delay).await;
        }
    });
}

async fn event_processing_loop(redis_pool: RedisPool) {
    let mut conn = redis_pool.get().await.unwrap();

    loop {
        if let Some(event) = conn.brpop("goal-set", 0f64).await.unwrap() {
            let event_handler_redis_pool = redis_pool.clone();
            tokio::spawn(async move {
                let _ = handle_goal_set_event(event_handler_redis_pool, event).await;
            });
        }
    }
}

#[tracing::instrument(skip(redis_pool), err)]
async fn handle_goal_set_event(redis_pool: RedisPool, event: [String; 2]) -> Result<(), AppError> {
    let str_value = event[1].to_owned();
    tracing::info!("Got GoalSetEvent event: {}", str_value);
    let goal_set_event = serde_json::from_str::<GoalSetEvent>(&str_value)?;
    let routine_recommended_event = RoutineRecommendedEvent {
        user_id: goal_set_event.user_id,
        routine_id: "str001".to_owned(),
    };
    let event_str = serde_json::to_string(&routine_recommended_event)?;
    let mut conn = redis_pool.get().await?;
    let list_length = conn.lpush("routine-recommended", event_str).await?;
    tracing::info!(
        "Published RoutineRecommendedEvent. Queued events: {}.",
        list_length
    );
    Ok(())
}
