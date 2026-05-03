use crate::config::AppConfig;
use crate::config::cors::configure_cors;
use crate::app::state::AppState;
use crate::app::router::create_router;
use crate::services::AlertService;
use crate::middleware::security_headers::security_headers_middleware;
use crate::middleware::logging::logging_middleware;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::PgPool;
use tower_governor::governor::GovernorConfigBuilder;
use std::sync::Arc;
use axum::middleware;
// use tokio::signal;  // Commented out - part of graceful shutdown
// use tokio::time::Duration;  // Commented out - part of graceful shutdown

pub async fn start_server(config: AppConfig, db_pool: PgPool) -> anyhow::Result<()> {
    let addr: SocketAddr = config.server_address().parse()?;
    let state = AppState::new(config, db_pool.clone()).await?;

    let email_service = state.get_email_service().clone();
    let pool = db_pool.clone();

    // Daily low stock check
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        loop {
            interval.tick().await;
            println!("Running daily low stock check...");
            if let Err(e) = AlertService::check_all_low_stock_products(&pool, &email_service).await {
                eprintln!("❌ Failed to send daily low stock alerts: {}", e);
            } else {
                println!("Daily low stock check completed");
            }
        }
    });

    // Rate limiter memory cleanup
    let limiter_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(60)
            .finish()
            .unwrap(),
    );
    let limiter = limiter_config.limiter().clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            limiter.retain_recent();
        }
    });

    let app = create_router(state)
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(configure_cors());

    tracing::info!("🚀 Server running on http://{}", addr);
    // tracing::info!("🛡️ Graceful shutdown enabled - Press Ctrl+C to stop");  // Commented out

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // GRACEFUL SHUTDOWN - Commented out for development
    // Will be re-enabled when ready for production
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        // .with_graceful_shutdown(shutdown_signal())  // Commented out
        .await?;

    tracing::info!("✨ Server shutdown complete");
    Ok(())
}

// GRACEFUL SHUTDOWN FUNCTION - Commented out for development
// Uncomment when ready to enable graceful shutdown
/*
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::info!("📡 Received SIGINT (Ctrl+C) signal");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
        tracing::info!("📡 Received SIGTERM signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🔄 Graceful shutdown initiated - waiting for in-flight requests to complete...");
    tracing::info!("⏱️  Maximum wait time: 30 seconds");
    
    // Give in-flight requests 30 seconds to complete
    tokio::time::sleep(Duration::from_secs(30)).await;
    
    tracing::info!("👋 Shutdown timeout reached - exiting now");
}
*/
