pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ProductFilter>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let limit = page_size as i64;
    let offset = (page - 1) as i64 * limit;
    
    let products = crate::repositories::ProductRepository::search(
        state.get_db_pool(),
        params.query.as_deref(),
        params.category_id,
        params.min_price,
        params.max_price,
        params.is_active,
        limit,
        offset,
    ).await?;
    
    // Get total count (you'll need to implement this in your repository)
    let total = crate::repositories::ProductRepository::get_total_count(
        state.get_db_pool(),
        params.query.as_deref(),
        params.category_id,
        params.min_price,
        params.max_price,
        params.is_active,
    ).await?;
    
    let product_responses: Vec<ProductResponse> = products.into_iter().map(|p| p.into()).collect();
    
    Ok(Json(PaginatedResponse::new(product_responses, total).with_pagination(page as i64, page_size as i64)))
}
