use axum::{routing::{get, post, put, delete}, Router, middleware};
use crate::handlers::{health, auth, product, cart, order, address, user};
use crate::middleware::auth::auth_middleware;
use super::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/change-password", post(auth::change_password))
        .route("/api/users/{id}", get(user::get_profile).put(user::update_profile))
        .route("/api/cart", get(cart::get_cart).post(cart::add_to_cart))
        .route("/api/cart/clear", delete(cart::clear_cart))
        .route("/api/cart/items/{item_id}", put(cart::update_cart_item).delete(cart::remove_from_cart))
        .route("/api/orders", post(order::checkout).get(order::get_my_orders))
        .route("/api/orders/{order_id}", get(order::get_order))
        .route("/api/admin/orders/{order_id}/status", put(order::update_order_status))
        .route("/api/addresses", get(address::get_addresses).post(address::create_address))
        .route("/api/addresses/{address_id}", put(address::update_address).delete(address::delete_address));

    let admin_routes = Router::new()
        .route("/api/admin/products", post(product::create_product))
        .route("/api/admin/products/{id}", put(product::update_product).delete(product::delete_product));

    Router::new()
        .route("/health", get(health::health_check))
        .route("/health/ready", get(health::readiness_check))
        .route("/health/live", get(health::liveness_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .route("/api/auth/forgot-password", post(auth::forgot_password))
        .route("/api/auth/reset-password", post(auth::reset_password))
        .route("/api/categories", get(product::list_categories))
        .route("/api/products/featured", get(product::get_featured_products))
        .route("/api/products/search", get(product::search_products))
        .route("/api/products", get(product::list_products))
        .route("/api/products/{id}", get(product::get_product))
        .merge(protected_routes.layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .merge(admin_routes.layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .with_state(state)
}