use axum::{
    extract::{State, Query, Path},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::{ProductFilter, ProductResponse, PaginatedResponse, CategoryResponse, ApiResponse};
use crate::errors::AppError;
use crate::repositories::ProductRepository;
use crate::repositories::CategoryRepository;

pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ProductFilter>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let limit = page_size as i64;
    let offset = (page - 1) as i64 * limit;
    
    // Only show active products to customers
    let is_active = Some(true);
    
    let products = ProductRepository::search(
        state.get_db_pool(),
        params.query.as_deref(),
        params.category_id,
        params.min_price,
        params.max_price,
        is_active,
        limit,
        offset,
    ).await?;
    
        let total = ProductRepository::count_search(
        state.get_db_pool(),
        params.query.as_deref(),
        params.category_id,
        params.min_price,
        params.max_price,
        is_active,
    ).await?;
    
    let product_responses: Vec<ProductResponse> = products.into_iter().map(|p| p.into()).collect();
    
    Ok(Json(PaginatedResponse::new(product_responses, total).with_pagination(page as i64, page_size as i64)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProductResponse>>, AppError> {
    let product = ProductRepository::find_by_id(state.get_db_pool(), &id).await?
        .ok_or_else(|| AppError::not_found("Product"))?;
    
    Ok(Json(ApiResponse::success(product.into())))
}

pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CategoryResponse>>>, AppError> {
    let categories = CategoryRepository::find_all_active(state.get_db_pool())
        .await?;
    Ok(Json(ApiResponse::success(
        categories.into_iter().map(|c| c.into()).collect()
    )))
}

pub async fn get_featured_products(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, AppError> {
    let products = ProductRepository::search(
        state.get_db_pool(),
        None,
        None,
        None,
        None,
        Some(true),
        8,
        0,
    ).await?;
    
    Ok(Json(ApiResponse::success(
        products.into_iter().map(|p| p.into()).collect()
    )))
}

pub async fn search_products(
    State(state): State<AppState>,
    Query(params): Query<ProductFilter>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, AppError> {
    let products = ProductRepository::search(
        state.get_db_pool(),
        params.query.as_deref(),
        None,
        None,
        None,
        Some(true),
        20,
        0,
    ).await?;
    
    Ok(Json(ApiResponse::success(
        products.into_iter().map(|p| p.into()).collect()
    )))
}
