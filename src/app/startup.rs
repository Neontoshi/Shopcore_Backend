use crate::config::AppConfig;
use crate::config::cors::configure_cors;
use crate::app::state::AppState;
use crate::app::router::create_router;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::PgPool;

pub async fn start_server(config: AppConfig, db_pool: PgPool) -> anyhow::Result<()> {
    let addr: SocketAddr = config.server_address().parse()?;  // Move this before config is moved
    let state = AppState::new(config, db_pool).await?;
    
    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(configure_cors());
    
    tracing::info!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}