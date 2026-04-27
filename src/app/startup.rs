use crate::config::AppConfig;
use crate::config::cors::configure_cors;
use crate::app::state::AppState;
use crate::app::router::create_router;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::PgPool;

pub async fn start_server(config: AppConfig, db_pool: PgPool) -> anyhow::Result<()> {
    let state = AppState::new(config.clone(), db_pool)?;
    
    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(configure_cors());
    
    let addr: SocketAddr = config.server_address().parse()?;
    
    tracing::info!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}