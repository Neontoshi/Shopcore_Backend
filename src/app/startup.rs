use crate::config::AppConfig;
use crate::config::cors::configure_cors;
use crate::app::state::AppState;
use crate::app::router::create_router;
use crate::services::AlertService;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::PgPool;

pub async fn start_server(config: AppConfig, db_pool: PgPool) -> anyhow::Result<()> {
    let addr: SocketAddr = config.server_address().parse()?;
    let state = AppState::new(config, db_pool.clone()).await?;
    
    let email_service = state.get_email_service().clone();
    let pool = db_pool.clone();
    
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
    
    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(configure_cors());
    
    tracing::info!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}