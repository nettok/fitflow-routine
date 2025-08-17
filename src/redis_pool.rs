use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

pub(crate) type RedisPool = Pool<RedisConnectionManager>;

pub async fn new_redis_pool(redis_url: &str) -> RedisPool {
    tracing::info!("connecting to redis");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
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
