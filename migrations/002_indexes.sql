-- Migration 003: All Indexes

-- Users
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_email_lower ON users(LOWER(email));
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_role_active ON users(role, is_active);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_users_last_login ON users(last_login_at DESC);
CREATE INDEX IF NOT EXISTS idx_users_active_customers ON users(id, email) WHERE is_active = true AND role = 'customer';
CREATE INDEX IF NOT EXISTS idx_users_cover_auth ON users(id, email, password_hash, role, is_active);

-- Categories
CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug);
CREATE INDEX IF NOT EXISTS idx_categories_display_order ON categories(display_order);
CREATE INDEX IF NOT EXISTS idx_categories_is_active ON categories(is_active);
CREATE INDEX IF NOT EXISTS idx_categories_parent_active ON categories(parent_id, is_active);
CREATE INDEX IF NOT EXISTS idx_categories_name_lower ON categories(LOWER(name));

-- Products
CREATE INDEX IF NOT EXISTS idx_products_category_id ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_slug ON products(slug);
CREATE INDEX IF NOT EXISTS idx_products_is_active ON products(is_active);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_price ON products(price);
CREATE INDEX IF NOT EXISTS idx_products_stock_quantity ON products(stock_quantity);
CREATE INDEX IF NOT EXISTS idx_products_category_active ON products(category_id, is_active);
CREATE INDEX IF NOT EXISTS idx_products_price_range ON products(price, is_active);
CREATE INDEX IF NOT EXISTS idx_products_created_price ON products(created_at DESC, price);
CREATE INDEX IF NOT EXISTS idx_products_active_low_stock ON products(id, stock_quantity) WHERE is_active = true AND stock_quantity < 10;
CREATE INDEX IF NOT EXISTS idx_products_cover_price_stock ON products(id, name, price, stock_quantity, is_active);
CREATE INDEX IF NOT EXISTS idx_products_name_trgm ON products USING gin(name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_products_description_trgm ON products USING gin(description gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_products_search_tsv ON products USING gin(to_tsvector('english', name || ' ' || COALESCE(description, '')));

-- Product images
CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id);
CREATE INDEX IF NOT EXISTS idx_product_images_is_primary ON product_images(is_primary);

-- Product variants
CREATE INDEX IF NOT EXISTS idx_product_variants_product_id ON product_variants(product_id);
CREATE INDEX IF NOT EXISTS idx_product_variants_sku ON product_variants(sku);

-- Reviews
CREATE INDEX IF NOT EXISTS idx_reviews_product_id ON reviews(product_id);
CREATE INDEX IF NOT EXISTS idx_reviews_user_id ON reviews(user_id);
CREATE INDEX IF NOT EXISTS idx_reviews_rating ON reviews(rating);
CREATE INDEX IF NOT EXISTS idx_reviews_created_at ON reviews(created_at);
CREATE INDEX IF NOT EXISTS idx_reviews_is_approved ON reviews(is_approved);
CREATE INDEX IF NOT EXISTS idx_reviews_verified_purchase ON reviews(is_verified_purchase);
CREATE INDEX IF NOT EXISTS idx_reviews_product_approved ON reviews(product_id, is_approved);
CREATE INDEX IF NOT EXISTS idx_reviews_product_rating ON reviews(product_id, rating);
CREATE INDEX IF NOT EXISTS idx_reviews_product_rating_approved ON reviews(product_id, rating, is_approved);
CREATE INDEX IF NOT EXISTS idx_reviews_created_approved_rating ON reviews(created_at DESC, is_approved, rating);
CREATE INDEX IF NOT EXISTS idx_reviews_helpful_ratio ON reviews(helpful_count, unhelpful_count);
CREATE INDEX IF NOT EXISTS idx_reviews_product_user ON reviews(product_id, user_id);
CREATE INDEX IF NOT EXISTS idx_review_helpfulness_review_id ON review_helpfulness(review_id);
CREATE INDEX IF NOT EXISTS idx_review_replies_review_id ON review_replies(review_id);

-- Addresses
CREATE INDEX IF NOT EXISTS idx_addresses_user_id ON addresses(user_id);
CREATE INDEX IF NOT EXISTS idx_addresses_user_default ON addresses(user_id, is_default);
CREATE INDEX IF NOT EXISTS idx_addresses_type ON addresses(address_type);
CREATE INDEX IF NOT EXISTS idx_addresses_postal_code ON addresses(postal_code);
CREATE INDEX IF NOT EXISTS idx_addresses_country ON addresses(country);
CREATE INDEX IF NOT EXISTS idx_addresses_deleted_at ON addresses(deleted_at);
CREATE INDEX IF NOT EXISTS idx_addresses_user_type ON addresses(user_id, address_type);
CREATE INDEX IF NOT EXISTS idx_addresses_user_default_type ON addresses(user_id, is_default, address_type);
CREATE INDEX IF NOT EXISTS idx_addresses_country_postal ON addresses(country, postal_code);
CREATE INDEX IF NOT EXISTS idx_addresses_user_verified ON addresses(user_id, is_verified);

-- Carts
CREATE INDEX IF NOT EXISTS idx_carts_user_id ON carts(user_id);
CREATE INDEX IF NOT EXISTS idx_carts_expires_at ON carts(expires_at);
CREATE INDEX IF NOT EXISTS idx_carts_user_active ON carts(user_id, expires_at);
CREATE INDEX IF NOT EXISTS idx_cart_items_cart_id ON cart_items(cart_id);
CREATE INDEX IF NOT EXISTS idx_cart_items_product_id ON cart_items(product_id);
CREATE INDEX IF NOT EXISTS idx_cart_items_cart_product ON cart_items(cart_id, product_id);

-- Coupons
CREATE INDEX IF NOT EXISTS idx_coupons_code ON coupons(code);
CREATE INDEX IF NOT EXISTS idx_coupons_is_active ON coupons(is_active);
CREATE INDEX IF NOT EXISTS idx_coupons_expires_at ON coupons(expires_at);
CREATE INDEX IF NOT EXISTS idx_coupon_usages_coupon_id ON coupon_usages(coupon_id);
CREATE INDEX IF NOT EXISTS idx_coupon_usages_user_id ON coupon_usages(user_id);
CREATE INDEX IF NOT EXISTS idx_coupon_usages_order_id ON coupon_usages(order_id);

-- Orders
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_order_number ON orders(order_number);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at);
CREATE INDEX IF NOT EXISTS idx_orders_user_status ON orders(user_id, status);
CREATE INDEX IF NOT EXISTS idx_orders_created_status ON orders(created_at DESC, status);
CREATE INDEX IF NOT EXISTS idx_orders_payment_status ON orders(payment_status, status);
CREATE INDEX IF NOT EXISTS idx_orders_total_range ON orders(total);
CREATE INDEX IF NOT EXISTS idx_orders_date_range ON orders(created_at) WHERE payment_status = 'paid';
CREATE INDEX IF NOT EXISTS idx_orders_order_number_like ON orders(order_number varchar_pattern_ops);
CREATE INDEX IF NOT EXISTS idx_orders_pending_payment ON orders(id, created_at) WHERE status = 'pending' AND payment_status = 'pending';
CREATE INDEX IF NOT EXISTS idx_orders_user_addresses ON orders(user_id, shipping_address_id, billing_address_id);
CREATE INDEX IF NOT EXISTS idx_orders_cover_summary ON orders(id, order_number, total, status, created_at, user_id);

-- Order items
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_product_id ON order_items(product_id);
CREATE INDEX IF NOT EXISTS idx_order_items_product_orders ON order_items(product_id, order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_quantity_price ON order_items(quantity, price);
CREATE INDEX IF NOT EXISTS idx_order_items_order_product ON order_items(order_id, product_id);

-- Payment transactions
CREATE INDEX IF NOT EXISTS idx_payment_transactions_order_id ON payment_transactions(order_id);
CREATE INDEX IF NOT EXISTS idx_payment_transactions_status ON payment_transactions(status);
CREATE INDEX IF NOT EXISTS idx_payment_transactions_provider_id ON payment_transactions(provider_transaction_id);

-- Wishlists
CREATE INDEX IF NOT EXISTS idx_wishlists_user_id ON wishlists(user_id);
CREATE INDEX IF NOT EXISTS idx_wishlists_product_id ON wishlists(product_id);

-- Product search view indexes
-- The view is created in migration 001_schema.sql
-- These indexes optimize search queries on the materialized view
CREATE INDEX IF NOT EXISTS idx_product_search_vector ON product_search_view USING gin(search_vector);
CREATE INDEX IF NOT EXISTS idx_product_search_name ON product_search_view(name);
CREATE INDEX IF NOT EXISTS idx_product_search_price ON product_search_view(price);

-- Analyze all tables for query planner
ANALYZE users;
ANALYZE products;
ANALYZE categories;
ANALYZE product_images;
ANALYZE product_variants;
ANALYZE reviews;
ANALYZE review_helpfulness;
ANALYZE review_replies;
ANALYZE addresses;
ANALYZE carts;
ANALYZE cart_items;
ANALYZE coupons;
ANALYZE coupon_usages;
ANALYZE orders;
ANALYZE order_items;
ANALYZE payment_transactions;
ANALYZE wishlists;