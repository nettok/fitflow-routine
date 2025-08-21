use crate::redis_pool::RedisPool;
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use redis::AsyncTypedCommands;
use serde::{Deserialize, Serialize};

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
        let mut conn = redis_pool.get().await.unwrap();
        loop {
            if let Some(event) = conn.brpop("goal-set", 0f64).await.unwrap_or(None) {
                handle_goal_set_event(&mut conn, event).await;
            }
        }
    });
}

#[tracing::instrument(skip(conn))]
async fn handle_goal_set_event(
    conn: &mut PooledConnection<'_, RedisConnectionManager>,
    event: [String; 2],
) {
    let str_value = event[1].to_owned();
    tracing::info!("Got event: {}", str_value);
    if let Ok(goal_set_event) = serde_json::from_str::<GoalSetEvent>(&str_value) {
        let routine_recommended_event = RoutineRecommendedEvent {
            user_id: goal_set_event.user_id,
            routine_id: "str001".to_owned(),
        };
        let event_str = serde_json::to_string(&routine_recommended_event).unwrap();
        if let Ok(list_length) = conn.lpush("routine-recommended", event_str).await {
            tracing::info!(
                "Published RoutineRecommendedEvent. Queued events: {}.",
                list_length
            );
        }
    }
}
