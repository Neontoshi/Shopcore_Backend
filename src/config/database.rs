use sqlx::{postgres::{PgPoolOptions}, PgPool, Error};
use std::time::Duration;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct DatabaseClient {
    pub pool: PgPool,
}

impl DatabaseClient {
    pub async fn new(config: &AppConfig) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&config.database_url)
            .await?;
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!()
        .run(pool)
        .await?;
    
    tracing::info!("Database migrations completed successfully");
    Ok(())
}