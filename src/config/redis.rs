use crate::config::AppConfig;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct RedisClient {
    pub connection_manager: ConnectionManager,
}

impl RedisClient {
    pub async fn new(config: &AppConfig) -> Result<Self, anyhow::Error> {
        let client = redis::Client::open(config.redis_url.as_str())?;
        let connection_manager = ConnectionManager::new(client).await?;
        
        Ok(Self {
            connection_manager,
        })
    }
}