use std::collections::HashMap;
use axum::{
    extract::{State, Path, Query, Extension},
    Json,
};
use uuid::Uuid;
use validator::Validate;
use crate::app::state::AppState;
use crate::dtos::review_dto::{CreateReviewRequest, CreateReplyRequest};
use crate::services::review_service::ReviewService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::types::PaginationParams;

pub async fn create_review(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateReviewRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let review = ReviewService::create_review(
        state.get_db_pool(),
        &auth_user.user_id,
        req,
    ).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": review
    })))
}

pub async fn get_product_reviews(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(50).max(1);

    let (reviews, total, summary) = ReviewService::get_product_reviews(
        state.get_db_pool(),
        &product_id,
        page,
        page_size,
    ).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": reviews,
        "total": total,
        "summary": summary,
        "page": page,
        "page_size": page_size
    })))
}

pub async fn check_user_review(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM reviews 
            WHERE user_id = $1 AND product_id = $2 AND deleted_at IS NULL
        ) as has_reviewed
        "#,
        auth_user.user_id,
        product_id
    )
    .fetch_one(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "has_reviewed": result.has_reviewed.unwrap_or(false)
    })))
}

pub async fn mark_review_helpful(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(review_id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let is_helpful = req.get("is_helpful").and_then(|v| v.as_bool()).ok_or_else(|| {
        AppError::bad_request("is_helpful is required")
    })?;

    ReviewService::mark_helpful(
        state.get_db_pool(),
        &review_id,
        &auth_user.user_id,
        is_helpful,
    ).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Thank you for your feedback"
    })))
}

pub async fn add_reply_to_review(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(review_id): Path<Uuid>,
    Json(req): Json<CreateReplyRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let reply = ReviewService::add_reply(
        state.get_db_pool(),
        &review_id,
        &auth_user.user_id,
        &req.reply,
    ).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": reply
    })))
}

pub async fn get_review_replies(
    State(state): State<AppState>,
    Path(review_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let replies = ReviewService::get_review_replies(
        state.get_db_pool(),
        &review_id,
    ).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": replies
    })))
}

pub async fn get_user_review_votes(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<Vec<Uuid>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut result = HashMap::new();
    for review_id in req {
        let row = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM review_helpfulness 
                WHERE review_id = $1 AND user_id = $2
            ) as "exists!"
            "#,
            review_id,
            auth_user.user_id
        )
        .fetch_one(state.get_db_pool())
        .await
        .map_err(|_| AppError::bad_request("Failed to check votes"))?;

        result.insert(review_id, row.exists);
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "data": result
    })))
}