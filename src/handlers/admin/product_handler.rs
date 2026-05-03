use axum::{
    extract::{State, Path, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

// Admin: Update product status (activate/deactivate)
pub async fn update_product_status(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Admin access required"));
    }

    let is_active = req.get("is_active").and_then(|v| v.as_bool()).ok_or_else(|| AppError::bad_request("is_active is required"))?;

    sqlx::query!(
        r#"
        UPDATE products SET is_active = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        is_active,
        product_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": format!("Product {} successfully", if is_active { "activated" } else { "deactivated" })
    })))
}