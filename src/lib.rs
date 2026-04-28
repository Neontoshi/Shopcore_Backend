pub mod app;
pub mod config;
pub mod constants;
pub mod errors;
pub mod types;
pub mod utils;
pub mod models;
pub mod dtos;
pub mod repositories;
pub mod handlers;
pub mod middleware;
pub mod services;

use anyhow::Context;
use app::startup::start_server;
use config::app_config::AppConfig;
use config::database::run_migrations;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() -> anyhow::Result<()> {
    // Load configuration
    let config = AppConfig::from_env().context("Failed to load configuration")?;
    tracing::info!("Configuration loaded");

    // Initialize logging
    init_logging(&config)?;

    // Initialize database pool
    let db_pool = init_database(&config).await?;

    // Run migrations
    run_migrations(&db_pool).await?;

    tracing::info!("Starting server...");
    
    // Start server (Redis is initialized inside AppState)
    start_server(config, db_pool).await?;

    Ok(())
}

fn init_logging(config: &AppConfig) -> anyhow::Result<()> {
    let env_filter = match config.app_env.as_str() {
        "development" => "debug",
        _ => "info",
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!(
                        "shopcore_backend={},axum::rejection={}",
                        env_filter, env_filter
                    )
                    .into()
                }),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    Ok(())
}

async fn init_database(config: &AppConfig) -> anyhow::Result<sqlx::PgPool> {
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    tracing::info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .context("Database connection test failed")?;

    tracing::info!("Database connected successfully");

    Ok(pool)
}