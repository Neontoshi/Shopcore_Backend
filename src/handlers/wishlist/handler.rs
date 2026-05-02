use axum::{
    extract::{State, Path, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::{ApiResponse, AddToWishlistRequest, WishlistItemResponse};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::repositories::WishlistRepository;

pub async fn add_to_wishlist(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<AddToWishlistRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    match WishlistRepository::add_item(state.get_db_pool(), &auth_user.user_id, &req.product_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(AppError::bad_request("Failed to add to wishlist or item already exists")),
    }
}

pub async fn remove_from_wishlist(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    match WishlistRepository::remove_item(state.get_db_pool(), &auth_user.user_id, &product_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(AppError::bad_request("Failed to remove from wishlist")),
    }
}

pub async fn get_wishlist(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<WishlistItemResponse>>>, AppError> {
    match WishlistRepository::get_user_wishlist(state.get_db_pool(), &auth_user.user_id).await {
        Ok(items) => {
            let responses: Vec<WishlistItemResponse> = items.into_iter().map(|item| WishlistItemResponse {
                id: item.id,
                product_id: item.product_id,
                name: item.name,
                slug: item.slug,
                price: item.price,
                compare_at_price: item.compare_at_price,
                image_url: item.image_url,
                average_rating: item.average_rating.to_string().parse::<f64>().unwrap_or(0.0),
                total_reviews: item.total_reviews,
                added_at: item.added_at,
            }).collect();
            Ok(Json(ApiResponse::success(responses)))
        }
        Err(_) => Err(AppError::bad_request("Failed to fetch wishlist")),
    }
}

pub async fn check_in_wishlist(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    match WishlistRepository::is_in_wishlist(state.get_db_pool(), &auth_user.user_id, &product_id).await {
        Ok(exists) => Ok(Json(ApiResponse::success(exists))),
        Err(_) => Err(AppError::bad_request("Failed to check wishlist")),
    }
}

pub async fn get_wishlist_count(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<i64>>, AppError> {
    match WishlistRepository::get_wishlist_count(state.get_db_pool(), &auth_user.user_id).await {
        Ok(count) => Ok(Json(ApiResponse::success(count))),
        Err(_) => Err(AppError::bad_request("Failed to get wishlist count")),
    }
}