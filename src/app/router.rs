use axum::{routing::{get, post, put, delete}, Router, middleware};
use crate::handlers::{health, auth, product, cart, order, address, user, vendor, admin, review, payments, webhook, wishlist};
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
        .route("/api/checkout", post(order::checkout))
        .route("/api/orders", post(order::checkout).get(order::get_my_orders))
        .route("/api/orders/{order_id}", get(order::get_order))
        .route("/api/orders/{order_id}/cancel", put(order::cancel_order))
        .route("/api/admin/orders/{order_id}/status", put(order::update_order_status))
        .route("/api/addresses", get(address::get_addresses).post(address::create_address))
        .route("/api/addresses/{address_id}", put(address::update_address).delete(address::delete_address))
        // Review routes (authenticated)
        .route("/api/reviews", post(review::create_review))
        .route("/api/reviews/user/{product_id}", get(review::check_user_review))
        .route("/api/reviews/{review_id}/helpful", post(review::mark_review_helpful))
        .route("/api/reviews/{review_id}/replies", post(review::add_reply_to_review))
        .route("/api/reviews/user-votes", post(review::get_user_review_votes))
        // Wishlist routes
        .route("/api/wishlist", get(wishlist::get_wishlist))
        .route("/api/wishlist", post(wishlist::add_to_wishlist))
        .route("/api/wishlist/{product_id}", delete(wishlist::remove_from_wishlist))
        .route("/api/wishlist/check/{product_id}", get(wishlist::check_in_wishlist))
        .route("/api/wishlist/count", get(wishlist::get_wishlist_count))
        // Admin routes (protected)
        .route("/api/admin/stats", get(admin::get_stats))
        .route("/api/admin/vendor/applications", get(admin::get_vendor_applications))
        .route("/api/admin/vendor/applications/{application_id}", put(admin::review_application))
        .route("/api/admin/users", get(admin::get_users))
        .route("/api/admin/users/{user_id}/status", put(admin::update_user_status))
        .route("/api/admin/products", get(admin::get_all_products))
        .route("/api/admin/orders", get(admin::get_all_orders))
        .route("/api/admin/orders/{order_id}/mark-paid", put(admin::mark_order_paid))
        // Vendor routes
        .route("/api/vendor/products", get(vendor::get_my_products).post(vendor::create_product))
        .route("/api/vendor/products/{product_id}", put(vendor::update_product).delete(vendor::delete_product))
        .route("/api/vendor/apply", post(vendor::apply_for_vendor))
        .route("/api/vendor/application", get(vendor::get_my_application))
        .route("/api/vendor/profile", get(vendor::get_my_vendor_profile))
        .route("/api/vendor/stats", get(vendor::get_vendor_stats))
        .route("/api/vendor/orders", get(vendor::get_vendor_orders))
        .route("/api/vendor/orders/{order_id}/status", put(vendor::update_order_status))
        
        // Payment routes
        .route("/api/payments/initiate", post(payments::initiate_payment))
        .route("/api/payments/crypto/status/{charge_id}", get(payments::get_crypto_status));
        
    let admin_routes = Router::new()
        .route("/api/admin/shipping/settings", get(admin::get_shipping_settings).put(admin::update_shipping_settings))
        .route("/api/admin/products/{product_id}/status", put(admin::update_product_status))
        .route("/api/admin/inventory", get(admin::get_inventory))
        .route("/api/admin/inventory/adjust", post(admin::manual_adjust_stock))
        .route("/api/admin/inventory/low-stock", get(admin::get_low_stock_summary));

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
        .route("/api/shipping/settings", get(admin::get_shipping_settings))

        // Public review routes
        .route("/api/reviews/product/{product_id}", get(review::get_product_reviews))
        .route("/api/reviews/{review_id}/replies", get(review::get_review_replies))
        
        // Webhook routes
        .route("/api/webhook/stripe", post(webhook::stripe_webhook))
        .route("/api/webhook/coinbase", post(webhook::coinbase_webhook))

        .merge(protected_routes.layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .merge(admin_routes.layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .with_state(state)
}