use axum::{
    extract::{State, Path, Query, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::{CheckoutRequest, OrderResponse, UpdateOrderStatusRequest};
use crate::services::OrderService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::types::PaginationParams;

pub async fn checkout(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CheckoutRequest>,
) -> Result<Json<crate::dtos::CheckoutResponse>, AppError> {
    let result = OrderService::checkout(
        state.get_db_pool(),
        &auth_user.user_id,
        req.shipping_address_id,
        req.billing_address_id,
        &req.payment_method,
        req.notes,
    ).await?;

    Ok(Json(result))
}

pub async fn get_order(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let is_admin = auth_user.role == "admin";
    let order = OrderService::get_order(
        state.get_db_pool(),
        &auth_user.user_id,
        &order_id,
        is_admin,
    ).await?;
    Ok(Json(order))
}

pub async fn get_my_orders(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    let (orders, _total) = OrderService::get_user_orders(
        state.get_db_pool(),
        &auth_user.user_id,
        page,
        page_size,
    ).await?;

    Ok(Json(orders))
}

pub async fn update_order_status(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
    Json(req): Json<UpdateOrderStatusRequest>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    OrderService::update_order_status(state.get_db_pool(), &order_id, req.status).await?;

    Ok(Json(crate::utils::MessageResponse::new("Order status updated successfully")))
}
pub async fn cancel_order(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    // Check if order belongs to user or user is admin
    let order = sqlx::query!(
        r#"
        SELECT user_id, status FROM orders WHERE id = $1
        "#,
        order_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Order"))?;

    if order.user_id != auth_user.user_id && auth_user.role != "admin" {
        return Err(AppError::forbidden("You don't have permission to cancel this order"));
    }

    if order.status != "pending" {
        return Err(AppError::bad_request("Only pending orders can be cancelled"));
    }

    sqlx::query!(
        r#"
        UPDATE orders SET status = 'cancelled', updated_at = NOW()
        WHERE id = $1
        "#,
        order_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(crate::utils::MessageResponse::new("Order cancelled successfully")))
}
