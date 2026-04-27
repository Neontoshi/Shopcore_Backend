use crate::config::AppConfig;

#[derive(Clone)]
pub struct RedisClient {
    // Will implement in Phase 3
}

impl RedisClient {
    pub async fn new(_config: &AppConfig) -> Result<Self, anyhow::Error> {
        // Placeholder for now
        Ok(Self {})
    }
}