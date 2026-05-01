-- ROOT CATEGORIES
INSERT INTO categories (id, name, slug, description, icon, display_order, is_active) VALUES
    (uuid_generate_v4(), 'Electronics', 'electronics', 'Latest electronics, gadgets, and tech accessories', '📱', 1, true),
    (uuid_generate_v4(), 'Clothing & Fashion', 'clothing-fashion', 'Trendy apparel, shoes, and accessories for men, women, and kids', '👕', 2, true),
    (uuid_generate_v4(), 'Home & Living', 'home-living', 'Furniture, decor, kitchenware, and home improvement', '🏠', 3, true),
    (uuid_generate_v4(), 'Books & Media', 'books-media', 'Books, e-books, audiobooks, and media', '📚', 4, true),
    (uuid_generate_v4(), 'Sports & Outdoors', 'sports-outdoors', 'Sports equipment, outdoor gear, and fitness accessories', '⚽', 5, true),
    (uuid_generate_v4(), 'Toys & Games', 'toys-games', 'Toys, board games, and video games for all ages', '🎮', 6, true),
    (uuid_generate_v4(), 'Health & Beauty', 'health-beauty', 'Personal care, cosmetics, and wellness products', '💄', 7, true),
    (uuid_generate_v4(), 'Automotive', 'automotive', 'Car parts, accessories, and maintenance tools', '🚗', 8, true),
    (uuid_generate_v4(), 'Pet Supplies', 'pet-supplies', 'Food, toys, and accessories for your furry friends', '🐕', 9, true),
    (uuid_generate_v4(), 'Baby & Kids', 'baby-kids', 'Baby gear, clothing, and nursery essentials', '👶', 10, true);

-- ELECTRONICS SUBCATEGORIES
DO $$
DECLARE
    electronics_id UUID;
    smartphones_id UUID;
    laptops_id UUID;
    audio_id UUID;
BEGIN
    SELECT id INTO electronics_id FROM categories WHERE slug = 'electronics';

    INSERT INTO categories (id, name, slug, description, parent_id, icon, display_order) VALUES
        (uuid_generate_v4(), 'Smartphones & Tablets', 'smartphones-tablets', 'Latest smartphones, tablets, and accessories', electronics_id, '📱', 1),
        (uuid_generate_v4(), 'Laptops & Computers', 'laptops-computers', 'Laptops, desktops, and computer components', electronics_id, '💻', 2),
        (uuid_generate_v4(), 'Audio & Headphones', 'audio-headphones', 'Headphones, speakers, and audio equipment', electronics_id, '🎧', 3),
        (uuid_generate_v4(), 'Cameras & Photography', 'cameras-photography', 'Digital cameras, lenses, and photography gear', electronics_id, '📷', 4),
        (uuid_generate_v4(), 'TV & Home Theater', 'tv-home-theater', 'Televisions, projectors, and home audio', electronics_id, '📺', 5),
        (uuid_generate_v4(), 'Wearable Technology', 'wearable-tech', 'Smartwatches, fitness trackers, and wearables', electronics_id, '⌚', 6);

    SELECT id INTO smartphones_id FROM categories WHERE slug = 'smartphones-tablets';
    SELECT id INTO laptops_id FROM categories WHERE slug = 'laptops-computers';
    SELECT id INTO audio_id FROM categories WHERE slug = 'audio-headphones';

    INSERT INTO categories (id, name, slug, description, parent_id, display_order) VALUES
        (uuid_generate_v4(), 'Phone Cases', 'phone-cases', 'Protective cases and covers', smartphones_id, 1),
        (uuid_generate_v4(), 'Screen Protectors', 'screen-protectors', 'Tempered glass and film protectors', smartphones_id, 2),
        (uuid_generate_v4(), 'Gaming Laptops', 'gaming-laptops', 'High-performance gaming laptops', laptops_id, 1),
        (uuid_generate_v4(), 'Business Laptops', 'business-laptops', 'Professional laptops for work', laptops_id, 2),
        (uuid_generate_v4(), 'Wireless Earbuds', 'wireless-earbuds', 'True wireless earbuds', audio_id, 1),
        (uuid_generate_v4(), 'Over-Ear Headphones', 'over-ear-headphones', 'Premium over-ear headphones', audio_id, 2);
END $$;

-- CLOTHING SUBCATEGORIES
DO $$
DECLARE
    clothing_id UUID;
BEGIN
    SELECT id INTO clothing_id FROM categories WHERE slug = 'clothing-fashion';

    INSERT INTO categories (id, name, slug, description, parent_id, icon, display_order) VALUES
        (uuid_generate_v4(), 'Men''s Fashion', 'mens-fashion', 'Clothing and accessories for men', clothing_id, '👔', 1),
        (uuid_generate_v4(), 'Women''s Fashion', 'womens-fashion', 'Clothing and accessories for women', clothing_id, '👗', 2),
        (uuid_generate_v4(), 'Kids'' Fashion', 'kids-fashion', 'Clothing for boys and girls', clothing_id, '🧒', 3),
        (uuid_generate_v4(), 'Shoes', 'shoes', 'Footwear for all occasions', clothing_id, '👟', 4),
        (uuid_generate_v4(), 'Accessories', 'accessories', 'Bags, jewelry, and fashion accessories', clothing_id, '👜', 5);
END $$;

-- HOME & LIVING SUBCATEGORIES
DO $$
DECLARE
    home_id UUID;
BEGIN
    SELECT id INTO home_id FROM categories WHERE slug = 'home-living';

    INSERT INTO categories (id, name, slug, description, parent_id, icon, display_order) VALUES
        (uuid_generate_v4(), 'Furniture', 'furniture', 'Sofas, beds, tables, and chairs', home_id, '🛋️', 1),
        (uuid_generate_v4(), 'Kitchen & Dining', 'kitchen-dining', 'Cookware, dinnerware, and kitchen gadgets', home_id, '🍳', 2),
        (uuid_generate_v4(), 'Bedding & Bath', 'bedding-bath', 'Sheets, towels, and bathroom accessories', home_id, '🛏️', 3),
        (uuid_generate_v4(), 'Home Decor', 'home-decor', 'Wall art, candles, and decorative items', home_id, '🖼️', 4),
        (uuid_generate_v4(), 'Appliances', 'appliances', 'Major and small home appliances', home_id, '🔌', 5),
        (uuid_generate_v4(), 'Tools & Home Improvement', 'tools-improvement', 'Hardware, tools, and DIY supplies', home_id, '🔧', 6);
END $$;

-- SPORTS SUBCATEGORIES
DO $$
DECLARE
    sports_id UUID;
BEGIN
    SELECT id INTO sports_id FROM categories WHERE slug = 'sports-outdoors';

    INSERT INTO categories (id, name, slug, description, parent_id, icon, display_order) VALUES
        (uuid_generate_v4(), 'Exercise & Fitness', 'exercise-fitness', 'Equipment for working out and staying fit', sports_id, '💪', 1),
        (uuid_generate_v4(), 'Outdoor Recreation', 'outdoor-recreation', 'Camping, hiking, and outdoor gear', sports_id, '⛺', 2),
        (uuid_generate_v4(), 'Team Sports', 'team-sports', 'Equipment for soccer, basketball, baseball, etc.', sports_id, '⚽', 3),
        (uuid_generate_v4(), 'Cycling', 'cycling', 'Bikes, parts, and cycling accessories', sports_id, '🚲', 4),
        (uuid_generate_v4(), 'Swimming', 'swimming', 'Swimwear, goggles, and pool accessories', sports_id, '🏊', 5);
END $$;

-- ADMIN USER - Password: admin123
INSERT INTO users (id, email, password_hash, first_name, last_name, phone_number, role, is_active)
VALUES (
    uuid_generate_v4(),
    'admin@shopcore.com',
    crypt('admin123', gen_salt('bf')),
    'Super',
    'Admin',
    '+1234567890',
    'admin',
    true
) ON CONFLICT (email) DO NOTHING;

-- TEST CUSTOMERS - Password: customer123 or test123
INSERT INTO users (id, email, password_hash, first_name, last_name, phone_number, role, is_active)
VALUES 
    (
        uuid_generate_v4(),
        'customer1@example.com',
        crypt('customer123', gen_salt('bf')),
        'John',
        'Doe',
        '+1234567890',
        'customer',
        true
    ),
    (
        uuid_generate_v4(),
        'vendor@example.com',
        crypt('customer123', gen_salt('bf')),
        'Jane',
        'Smith',
        '+1987654321',
        'vendor',
        true
    ),
    (
        uuid_generate_v4(),
        'test@example.com',
        crypt('test123', gen_salt('bf')),
        'Test',
        'User',
        '+1122334455',
        'customer',
        true
    )
ON CONFLICT (email) DO NOTHING;
