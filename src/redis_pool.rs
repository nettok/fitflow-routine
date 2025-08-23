use crate::errors::AppError;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub(crate) type RedisPool = Pool<RedisConnectionManager>;

pub async fn new_redis_pool(redis_url: &str) -> Result<RedisPool, AppError> {
    let manager = RedisConnectionManager::new(redis_url)?;
    Ok(Pool::builder().build(manager).await?)
}
