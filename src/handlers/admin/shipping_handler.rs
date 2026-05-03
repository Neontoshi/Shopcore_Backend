use axum::{
    extract::{State, Extension},
    Json,
};
use crate::app::state::AppState;
use crate::dtos::shipping_settings_dto::{UpdateShippingSettingsRequest, ShippingSettingsResponse};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

pub async fn get_shipping_settings(
    State(state): State<AppState>,
) -> Result<Json<ShippingSettingsResponse>, AppError> {
    let settings = sqlx::query!(
        r#"
        SELECT rate_per_kg, free_shipping_threshold, updated_at
        FROM shipping_settings
        WHERE is_active = true
        ORDER BY created_at DESC
        LIMIT 1
        "#
    )
    .fetch_one(state.get_db_pool())
    .await?;

    Ok(Json(ShippingSettingsResponse {
        rate_per_kg: settings.rate_per_kg,
        free_shipping_threshold: settings.free_shipping_threshold,
        updated_at: settings.updated_at.unwrap_or_else(chrono::Utc::now),
    }))
}

pub async fn update_shipping_settings(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdateShippingSettingsRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Admin access required"));
    }

    let current = sqlx::query!(
        r#"
        SELECT rate_per_kg, free_shipping_threshold
        FROM shipping_settings
        WHERE is_active = true
        ORDER BY created_at DESC
        LIMIT 1
        "#
    )
    .fetch_one(state.get_db_pool())
    .await?;

    let new_rate = req.rate_per_kg.unwrap_or(current.rate_per_kg);
    let new_threshold = req.free_shipping_threshold.unwrap_or(current.free_shipping_threshold);

    sqlx::query!(
        r#"
        INSERT INTO shipping_settings (rate_per_kg, free_shipping_threshold, updated_by)
        VALUES ($1, $2, $3)
        "#,
        new_rate,
        new_threshold,
        auth_user.user_id
    )
    .execute(state.get_db_pool())
    .await?;

    sqlx::query!(
        r#"
        UPDATE shipping_settings 
        SET is_active = false 
        WHERE is_active = true AND id != (
            SELECT id FROM shipping_settings 
            WHERE is_active = true 
            ORDER BY created_at DESC 
            LIMIT 1
        )
        "#
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Shipping settings updated successfully",
        "rate_per_kg": new_rate,
        "free_shipping_threshold": new_threshold
    })))
}