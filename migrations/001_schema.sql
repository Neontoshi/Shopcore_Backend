-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- USERS
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone_number VARCHAR(20),
    role VARCHAR(20) NOT NULL DEFAULT 'customer',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE
);

-- CATEGORIES
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    parent_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    icon VARCHAR(255),
    image_url TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    meta_title VARCHAR(255),
    meta_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PRODUCTS
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    compare_at_price DECIMAL(10, 2) CHECK (compare_at_price >= 0),
    stock_quantity INTEGER NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    sku VARCHAR(100) UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    image_url TEXT,
    average_rating DECIMAL(3, 2) NOT NULL DEFAULT 0,
    total_reviews INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PRODUCT IMAGES
CREATE TABLE product_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    alt_text VARCHAR(255),
    display_order INTEGER NOT NULL DEFAULT 0,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PRODUCT VARIANTS
CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    sku VARCHAR(100) UNIQUE,
    price DECIMAL(10, 2),
    stock_quantity INTEGER NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    attributes JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- REVIEWS
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    comment TEXT,
    images TEXT[] DEFAULT '{}',
    is_verified_purchase BOOLEAN NOT NULL DEFAULT false,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    unhelpful_count INTEGER NOT NULL DEFAULT 0,
    is_approved BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(user_id, product_id)
);

CREATE TABLE review_helpfulness (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_helpful BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(review_id, user_id)
);

CREATE TABLE review_replies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    review_id UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reply TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ADDRESSES
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    address_type VARCHAR(20) NOT NULL CHECK (address_type IN ('shipping', 'billing', 'both')),
    is_default BOOLEAN NOT NULL DEFAULT false,
    address_line1 VARCHAR(255) NOT NULL,
    address_line2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    recipient_name VARCHAR(255),
    phone_number VARCHAR(20),
    email VARCHAR(255),
    company_name VARCHAR(255),
    tax_id VARCHAR(100),
    delivery_instructions TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- CARTS
CREATE TABLE carts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '7 days'
);

CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cart_id UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price_at_add DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(cart_id, product_id)
);

-- COUPONS
CREATE TABLE coupons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    discount_type VARCHAR(20) NOT NULL CHECK (discount_type IN ('percentage', 'fixed')),
    discount_value DECIMAL(10, 2) NOT NULL CHECK (discount_value > 0),
    minimum_order_amount DECIMAL(10, 2),
    maximum_discount_amount DECIMAL(10, 2),
    usage_limit INTEGER,
    usage_count INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    starts_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ORDERS
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    order_number VARCHAR(50) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    subtotal DECIMAL(10, 2) NOT NULL,
    tax DECIMAL(10, 2) NOT NULL,
    shipping_cost DECIMAL(10, 2) NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    discount_amount DECIMAL(10, 2) NOT NULL DEFAULT 0,
    coupon_id UUID REFERENCES coupons(id),
    shipping_address_id UUID NOT NULL REFERENCES addresses(id),
    billing_address_id UUID NOT NULL REFERENCES addresses(id),
    payment_method VARCHAR(50) NOT NULL,
    payment_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price DECIMAL(10, 2) NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    product_sku VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE coupon_usages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    coupon_id UUID NOT NULL REFERENCES coupons(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    discount_amount DECIMAL(10, 2) NOT NULL,
    used_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(coupon_id, order_id)
);

-- PAYMENT TRANSACTIONS
CREATE TABLE payment_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status VARCHAR(20) NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')),
    payment_method VARCHAR(50) NOT NULL,
    provider_transaction_id VARCHAR(255),
    provider_response JSONB,
    failure_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- WISHLISTS
CREATE TABLE wishlists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, product_id)
);

-- MATERIALIZED VIEW: Product Search
CREATE MATERIALIZED VIEW product_search_view AS
SELECT 
    p.id,
    p.name,
    p.description,
    p.price,
    p.compare_at_price,
    p.sku,
    p.is_active,
    p.category_id,
    p.average_rating,
    p.total_reviews,
    p.stock_quantity,
    c.name as category_name,
    c.slug as category_slug,
    to_tsvector('english', p.name || ' ' || COALESCE(p.description, '')) as search_vector
FROM products p
LEFT JOIN categories c ON p.category_id = c.id
WHERE p.is_active = true;

-- Create unique index for materialized view to enable concurrent refresh
CREATE UNIQUE INDEX idx_product_search_view_id ON product_search_view(id);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- UPDATED_AT TRIGGERS
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_product_variants_updated_at BEFORE UPDATE ON product_variants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reviews_updated_at BEFORE UPDATE ON reviews
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_review_replies_updated_at BEFORE UPDATE ON review_replies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_addresses_updated_at BEFORE UPDATE ON addresses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_carts_updated_at BEFORE UPDATE ON carts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_cart_items_updated_at BEFORE UPDATE ON cart_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_coupons_updated_at BEFORE UPDATE ON coupons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_payment_transactions_updated_at BEFORE UPDATE ON payment_transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- CATEGORY FUNCTIONS & TRIGGERS

-- Auto slug generation
CREATE OR REPLACE FUNCTION generate_category_slug()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.slug IS NULL OR NEW.slug = '' THEN
        NEW.slug := LOWER(REGEXP_REPLACE(NEW.name, '[^a-zA-Z0-9]+', '-', 'g'));
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_generate_category_slug
    BEFORE INSERT ON categories
    FOR EACH ROW EXECUTE FUNCTION generate_category_slug();

-- Category tree function
CREATE OR REPLACE FUNCTION get_category_tree(p_parent_id UUID DEFAULT NULL)
RETURNS TABLE(
    id UUID,
    name VARCHAR,
    slug VARCHAR,
    description TEXT,
    parent_id UUID,
    icon VARCHAR,
    image_url TEXT,
    display_order INTEGER,
    level INTEGER,
    path TEXT[]
) AS $$
BEGIN
    RETURN QUERY
    WITH RECURSIVE category_tree AS (
        SELECT
            c.id, c.name, c.slug, c.description, c.parent_id,
            c.icon, c.image_url, c.display_order,
            0 AS level, ARRAY[c.slug] AS path
        FROM categories c
        WHERE (p_parent_id IS NULL AND c.parent_id IS NULL)
           OR (c.parent_id = p_parent_id)
        UNION ALL
        SELECT
            c.id, c.name, c.slug, c.description, c.parent_id,
            c.icon, c.image_url, c.display_order,
            ct.level + 1, ct.path || c.slug
        FROM categories c
        INNER JOIN category_tree ct ON c.parent_id = ct.id
        WHERE c.is_active = true
    )
    SELECT * FROM category_tree ORDER BY display_order, name;
END;
$$ LANGUAGE plpgsql;

-- Category breadcrumb function
CREATE OR REPLACE FUNCTION get_category_breadcrumb(p_category_id UUID)
RETURNS TABLE(id UUID, name VARCHAR, slug VARCHAR, level INTEGER) AS $$
BEGIN
    RETURN QUERY
    WITH RECURSIVE breadcrumb AS (
        SELECT c.id, c.name, c.slug, 0 AS level
        FROM categories c WHERE c.id = p_category_id
        UNION ALL
        SELECT c.id, c.name, c.slug, b.level + 1
        FROM categories c
        INNER JOIN breadcrumb b ON c.id = b.id
        WHERE c.parent_id IS NOT NULL
    )
    SELECT * FROM breadcrumb ORDER BY level DESC;
END;
$$ LANGUAGE plpgsql;

-- Category navigation view
CREATE OR REPLACE VIEW category_navigation AS
WITH RECURSIVE category_paths AS (
    SELECT id, name, slug, parent_id,
           name::TEXT AS path_names,
           slug::TEXT AS path_slugs,
           1 AS depth
    FROM categories
    WHERE parent_id IS NULL AND is_active = true
    UNION ALL
    SELECT c.id, c.name, c.slug, c.parent_id,
           cp.path_names || ' > ' || c.name,
           cp.path_slugs || '/' || c.slug,
           cp.depth + 1
    FROM categories c
    INNER JOIN category_paths cp ON c.parent_id = cp.id
    WHERE c.is_active = true
)
SELECT * FROM category_paths ORDER BY path_names;

-- REVIEW FUNCTIONS & TRIGGERS

-- Update product average rating
CREATE OR REPLACE FUNCTION update_product_average_rating()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE products
    SET
        average_rating = (
            SELECT COALESCE(AVG(rating), 0)
            FROM reviews
            WHERE product_id = NEW.product_id AND is_approved = true AND deleted_at IS NULL
        ),
        total_reviews = (
            SELECT COUNT(*)
            FROM reviews
            WHERE product_id = NEW.product_id AND is_approved = true AND deleted_at IS NULL
        ),
        updated_at = NOW()
    WHERE id = NEW.product_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_product_rating_after_insert
    AFTER INSERT ON reviews FOR EACH ROW
    EXECUTE FUNCTION update_product_average_rating();

CREATE TRIGGER trigger_update_product_rating_after_update
    AFTER UPDATE OF rating, is_approved ON reviews FOR EACH ROW
    EXECUTE FUNCTION update_product_average_rating();

CREATE TRIGGER trigger_update_product_rating_after_delete
    AFTER DELETE ON reviews FOR EACH ROW
    EXECUTE FUNCTION update_product_average_rating();

-- Auto-verify review as purchased
CREATE OR REPLACE FUNCTION mark_review_as_verified()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM order_items oi
        JOIN orders o ON oi.order_id = o.id
        WHERE oi.product_id = NEW.product_id
          AND o.user_id = NEW.user_id
          AND o.payment_status = 'paid'
          AND o.status IN ('delivered', 'completed')
    ) THEN
        NEW.is_verified_purchase := true;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_verify_review_purchase
    BEFORE INSERT ON reviews FOR EACH ROW
    EXECUTE FUNCTION mark_review_as_verified();

-- Update helpfulness counts
CREATE OR REPLACE FUNCTION update_review_helpfulness()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NEW.is_helpful THEN
            UPDATE reviews SET helpful_count = helpful_count + 1, updated_at = NOW() WHERE id = NEW.review_id;
        ELSE
            UPDATE reviews SET unhelpful_count = unhelpful_count + 1, updated_at = NOW() WHERE id = NEW.review_id;
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        IF OLD.is_helpful THEN
            UPDATE reviews SET helpful_count = helpful_count - 1, updated_at = NOW() WHERE id = OLD.review_id;
        ELSE
            UPDATE reviews SET unhelpful_count = unhelpful_count - 1, updated_at = NOW() WHERE id = OLD.review_id;
        END IF;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_helpfulness_on_insert
    AFTER INSERT ON review_helpfulness FOR EACH ROW
    EXECUTE FUNCTION update_review_helpfulness();

CREATE TRIGGER trigger_update_helpfulness_on_delete
    AFTER DELETE ON review_helpfulness FOR EACH ROW
    EXECUTE FUNCTION update_review_helpfulness();

-- Product rating summary function
CREATE OR REPLACE FUNCTION get_product_rating_summary(p_product_id UUID)
RETURNS TABLE(
    average_rating DECIMAL(3,2),
    total_reviews BIGINT,
    rating_5_stars BIGINT,
    rating_4_stars BIGINT,
    rating_3_stars BIGINT,
    rating_2_stars BIGINT,
    rating_1_stars BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COALESCE(AVG(rating), 0)::DECIMAL(3,2),
        COUNT(*),
        COUNT(*) FILTER (WHERE rating = 5),
        COUNT(*) FILTER (WHERE rating = 4),
        COUNT(*) FILTER (WHERE rating = 3),
        COUNT(*) FILTER (WHERE rating = 2),
        COUNT(*) FILTER (WHERE rating = 1)
    FROM reviews
    WHERE product_id = p_product_id AND is_approved = true AND deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;

-- ADDRESS FUNCTIONS & TRIGGERS

-- Ensure single default address per user per type
CREATE OR REPLACE FUNCTION ensure_single_default_address()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_default THEN
        IF NEW.address_type IN ('shipping', 'both') THEN
            UPDATE addresses SET is_default = false
            WHERE user_id = NEW.user_id AND id != NEW.id
              AND address_type IN ('shipping', 'both') AND is_default = true;
        END IF;
        IF NEW.address_type IN ('billing', 'both') THEN
            UPDATE addresses SET is_default = false
            WHERE user_id = NEW.user_id AND id != NEW.id
              AND address_type IN ('billing', 'both') AND is_default = true;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_ensure_single_default_address
    BEFORE INSERT OR UPDATE ON addresses FOR EACH ROW
    EXECUTE FUNCTION ensure_single_default_address();

-- Get user default address
CREATE OR REPLACE FUNCTION get_user_default_address(p_user_id UUID, addr_type VARCHAR DEFAULT 'shipping')
RETURNS TABLE(
    id UUID, address_line1 VARCHAR, address_line2 VARCHAR,
    city VARCHAR, state VARCHAR, postal_code VARCHAR,
    country VARCHAR, recipient_name VARCHAR, phone_number VARCHAR
) AS $$
BEGIN
    RETURN QUERY
    SELECT a.id, a.address_line1, a.address_line2, a.city, a.state,
           a.postal_code, a.country, a.recipient_name, a.phone_number
    FROM addresses a
    WHERE a.user_id = p_user_id AND a.deleted_at IS NULL
      AND (
          (addr_type = 'shipping' AND a.address_type IN ('shipping', 'both')) OR
          (addr_type = 'billing'  AND a.address_type IN ('billing',  'both'))
      )
    ORDER BY a.is_default DESC, a.created_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Format address for display
CREATE OR REPLACE FUNCTION format_address(p_address_id UUID)
RETURNS TEXT AS $$
DECLARE
    addr addresses%ROWTYPE;
    formatted TEXT;
BEGIN
    SELECT * INTO addr FROM addresses WHERE id = p_address_id AND deleted_at IS NULL;
    IF NOT FOUND THEN RETURN NULL; END IF;
    formatted := addr.address_line1;
    IF addr.address_line2 IS NOT NULL AND addr.address_line2 != '' THEN
        formatted := formatted || ', ' || addr.address_line2;
    END IF;
    formatted := formatted || ', ' || addr.city || ', ' || addr.state || ' ' || addr.postal_code;
    formatted := formatted || ', ' || addr.country;
    RETURN formatted;
END;
$$ LANGUAGE plpgsql;

-- Soft delete address
CREATE OR REPLACE FUNCTION soft_delete_address(p_address_id UUID)
RETURNS BOOLEAN AS $$
BEGIN
    UPDATE addresses SET deleted_at = NOW() WHERE id = p_address_id;
    RETURN FOUND;
END;
$$ LANGUAGE plpgsql;

-- User addresses view
CREATE OR REPLACE VIEW user_addresses_view AS
SELECT
    a.id, a.user_id,
    u.email AS user_email,
    u.first_name || ' ' || u.last_name AS user_name,
    a.address_type, a.is_default,
    a.address_line1, a.address_line2,
    a.city, a.state, a.postal_code, a.country,
    a.recipient_name, a.phone_number,
    a.company_name, a.delivery_instructions,
    a.is_verified, a.created_at,
    format_address(a.id) AS formatted_address
FROM addresses a
JOIN users u ON a.user_id = u.id
WHERE a.deleted_at IS NULL;

-- Refresh materialized view function
CREATE OR REPLACE FUNCTION refresh_product_search_view()
RETURNS TRIGGER AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY product_search_view;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_refresh_search_on_product_change
    AFTER INSERT OR UPDATE OR DELETE ON products
    FOR EACH STATEMENT EXECUTE FUNCTION refresh_product_search_view();

CREATE TRIGGER trigger_refresh_search_on_category_change
    AFTER UPDATE ON categories
    FOR EACH STATEMENT EXECUTE FUNCTION refresh_product_search_view();

-- Auto-create cart when a new user is created
CREATE OR REPLACE FUNCTION create_user_cart()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO carts (user_id, created_at, updated_at, expires_at)
    VALUES (NEW.id, NOW(), NOW(), NOW() + INTERVAL '7 days');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_create_user_cart ON users;

CREATE TRIGGER trigger_create_user_cart
    AFTER INSERT ON users
    FOR EACH ROW
    EXECUTE FUNCTION create_user_cart();