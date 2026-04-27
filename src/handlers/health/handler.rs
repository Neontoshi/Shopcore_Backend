use axum::{Json, http::StatusCode, extract::State};
use serde_json::json;
use crate::app::state::AppState;

pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    )
}

pub async fn readiness_check(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1")
        .fetch_one(state.get_db_pool())
        .await
    {
        Ok(_) => "connected",
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            "disconnected"
        }
    };
    
    let status_code = if db_status == "connected" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    
    (
        status_code,
        Json(json!({
            "status": if status_code.is_success() { "ready" } else { "not_ready" },
            "checks": {
                "database": db_status,
                "redis": "not_configured", // Will be updated in Phase 3
            },
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    )
}

pub async fn liveness_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "alive",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    )
}