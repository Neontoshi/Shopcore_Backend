-- PRODUCT SEED DATA

-- Get category IDs for product insertion
DO $$
DECLARE
    electronics_id UUID;
    smartphones_id UUID;
    laptops_id UUID;
    gaming_laptops_id UUID;
    audio_id UUID;
    wireless_earbuds_id UUID;
    cameras_id UUID;
    wearables_id UUID;
    
    furniture_id UUID;
    kitchen_id UUID;
    bedding_id UUID;
    decor_id UUID;
    
    exercise_id UUID;
    outdoor_id UUID;
    cycling_id UUID;
    
BEGIN
    -- Fetch Electronics category IDs
    SELECT id INTO electronics_id FROM categories WHERE slug = 'electronics';
    SELECT id INTO smartphones_id FROM categories WHERE slug = 'smartphones-tablets';
    SELECT id INTO laptops_id FROM categories WHERE slug = 'laptops-computers';
    SELECT id INTO gaming_laptops_id FROM categories WHERE slug = 'gaming-laptops';
    SELECT id INTO audio_id FROM categories WHERE slug = 'audio-headphones';
    SELECT id INTO wireless_earbuds_id FROM categories WHERE slug = 'wireless-earbuds';
    SELECT id INTO cameras_id FROM categories WHERE slug = 'cameras-photography';
    SELECT id INTO wearables_id FROM categories WHERE slug = 'wearable-tech';
    
    -- Fetch Home & Living category IDs
    SELECT id INTO furniture_id FROM categories WHERE slug = 'furniture';
    SELECT id INTO kitchen_id FROM categories WHERE slug = 'kitchen-dining';
    SELECT id INTO bedding_id FROM categories WHERE slug = 'bedding-bath';
    SELECT id INTO decor_id FROM categories WHERE slug = 'home-decor';
    
    -- Fetch Sports category IDs
    SELECT id INTO exercise_id FROM categories WHERE slug = 'exercise-fitness';
    SELECT id INTO outdoor_id FROM categories WHERE slug = 'outdoor-recreation';
    SELECT id INTO cycling_id FROM categories WHERE slug = 'cycling';

    -- ELECTRONICS PRODUCTS
    
    -- Smartphones & Tablets
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'iPhone 15 Pro Max', 'iphone-15-pro-max', 'Latest flagship iPhone with titanium design, A17 Pro chip, and advanced camera system', 1199.99, 1299.99, 45, smartphones_id, 'IPH-15PM-256', true, 'https://images.unsplash.com/photo-1592286927505-5588f5e0f02e?w=800'),
        (uuid_generate_v4(), 'Samsung Galaxy S24 Ultra', 'samsung-galaxy-s24-ultra', 'Premium Android flagship with 200MP camera, S Pen, and AI features', 1099.99, 1199.99, 38, smartphones_id, 'SAM-S24U-512', true, 'https://images.unsplash.com/photo-1610945415295-d9bbf067e59c?w=800'),
        (uuid_generate_v4(), 'iPad Pro 12.9" M2', 'ipad-pro-12-m2', 'Professional tablet with M2 chip, Liquid Retina XDR display, and Apple Pencil support', 899.99, 999.99, 28, smartphones_id, 'IPD-PR12-256', true, 'https://images.unsplash.com/photo-1544244015-0df4b3ffc6b0?w=800'),
        (uuid_generate_v4(), 'Google Pixel 8 Pro', 'google-pixel-8-pro', 'AI-powered flagship with exceptional camera and clean Android experience', 899.99, null, 52, smartphones_id, 'GPX-8P-256', true, 'https://images.unsplash.com/photo-1598327105666-5b89351aff97?w=800'),
        (uuid_generate_v4(), 'OnePlus 12', 'oneplus-12', 'Flagship killer with Snapdragon 8 Gen 3, fast charging, and smooth display', 749.99, 849.99, 60, smartphones_id, 'OP-12-256', true, 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=800');

    -- Gaming Laptops
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'ASUS ROG Strix G16', 'asus-rog-strix-g16', 'Powerful gaming laptop with RTX 4070, Intel i9-13980HX, 16GB RAM, 1TB SSD', 1899.99, 2199.99, 15, gaming_laptops_id, 'ASU-ROG-G16', true, 'https://images.unsplash.com/photo-1603302576837-37561b2e2302?w=800'),
        (uuid_generate_v4(), 'MSI Katana 15', 'msi-katana-15', 'Budget gaming beast with RTX 4060, i7-13620H, 144Hz display, 512GB SSD', 1199.99, 1399.99, 22, gaming_laptops_id, 'MSI-KAT-15', true, 'https://images.unsplash.com/photo-1625842268584-8f3296236761?w=800'),
        (uuid_generate_v4(), 'Razer Blade 15', 'razer-blade-15', 'Premium gaming ultrabook with RTX 4080, QHD 240Hz, sleek aluminum design', 2799.99, 2999.99, 8, gaming_laptops_id, 'RZR-BLD-15', true, 'https://images.unsplash.com/photo-1587202372634-32705e3bf49c?w=800'),
        (uuid_generate_v4(), 'Lenovo Legion Pro 7i', 'lenovo-legion-pro-7i', 'High-performance gaming with RTX 4090, i9-13900HX, 32GB RAM, RGB keyboard', 3299.99, 3499.99, 6, gaming_laptops_id, 'LEN-LP7-RTX', true, 'https://images.unsplash.com/photo-1593642632823-8f785ba67e45?w=800'),
        (uuid_generate_v4(), 'Acer Predator Helios 300', 'acer-predator-helios-300', 'Mid-range gaming powerhouse with RTX 4050, i7-13700H, excellent cooling', 1399.99, 1599.99, 18, gaming_laptops_id, 'ACR-PH300', true, 'https://images.unsplash.com/photo-1588872657578-7efd1f1555ed?w=800'),
        (uuid_generate_v4(), 'Alienware m18', 'alienware-m18', 'Desktop replacement gaming monster with RTX 4090, i9-13900HX, 18" QHD+ display', 3799.99, null, 5, gaming_laptops_id, 'ALN-M18-4090', true, 'https://images.unsplash.com/photo-1616763355548-1b606f439f86?w=800');

    -- Audio & Headphones
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Sony WH-1000XM5', 'sony-wh1000xm5', 'Industry-leading noise cancellation with exceptional sound quality and 30hr battery', 349.99, 399.99, 75, audio_id, 'SNY-WH1000XM5', true, 'https://images.unsplash.com/photo-1618366712010-f4ae9c647dcf?w=800'),
        (uuid_generate_v4(), 'Bose QuietComfort Ultra', 'bose-qc-ultra', 'Premium ANC headphones with spatial audio and all-day comfort', 379.99, 429.99, 42, audio_id, 'BSE-QCU-BLK', true, 'https://images.unsplash.com/photo-1546435770-a3e426bf472b?w=800'),
        (uuid_generate_v4(), 'Apple AirPods Max', 'airpods-max', 'Luxury over-ear headphones with computational audio and seamless Apple integration', 479.99, 549.99, 30, audio_id, 'APL-APM-SLV', true, 'https://images.unsplash.com/photo-1625948515291-69613efd103f?w=800'),
        (uuid_generate_v4(), 'Sennheiser Momentum 4', 'sennheiser-momentum-4', 'Audiophile-grade wireless headphones with 60hr battery and adaptive ANC', 329.99, 379.99, 35, audio_id, 'SEN-MOM4-BLK', true, 'https://images.unsplash.com/photo-1585298723682-7115561c51b7?w=800');

    -- Wireless Earbuds
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Apple AirPods Pro 2', 'airpods-pro-2', 'Premium earbuds with adaptive ANC, spatial audio, and MagSafe charging', 249.99, null, 120, wireless_earbuds_id, 'APL-APP2-WHT', true, 'https://images.unsplash.com/photo-1606841837239-c5a1a4a07af7?w=800'),
        (uuid_generate_v4(), 'Samsung Galaxy Buds2 Pro', 'galaxy-buds2-pro', 'Hi-Fi sound with intelligent ANC and 360° audio for Galaxy devices', 179.99, 229.99, 95, wireless_earbuds_id, 'SAM-GB2P-GRP', true, 'https://images.unsplash.com/photo-1590658268037-6bf12165a8df?w=800'),
        (uuid_generate_v4(), 'Sony WF-1000XM5', 'sony-wf1000xm5', 'Best-in-class noise cancellation in ultra-compact earbuds', 279.99, 299.99, 68, wireless_earbuds_id, 'SNY-WF1000XM5', true, 'https://images.unsplash.com/photo-1572536147248-ac59a8abfa4b?w=800'),
        (uuid_generate_v4(), 'Bose QuietComfort Earbuds II', 'bose-qc-earbuds-2', 'Custom-fit earbuds with personalized noise cancellation', 249.99, 279.99, 54, wireless_earbuds_id, 'BSE-QCEB2', true, 'https://images.unsplash.com/photo-1590658165737-15a047b7a0a5?w=800'),
        (uuid_generate_v4(), 'Jabra Elite 85t', 'jabra-elite-85t', 'Professional earbuds with advanced ANC and multipoint connectivity', 199.99, 249.99, 72, wireless_earbuds_id, 'JBR-E85T-BLK', true, 'https://images.unsplash.com/photo-1606220588913-b3aacb4d2f46?w=800');

    -- Cameras
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Canon EOS R6 Mark II', 'canon-eos-r6-ii', 'Full-frame mirrorless camera with 24MP sensor and advanced autofocus', 2499.99, null, 12, cameras_id, 'CAN-R6M2-BDY', true, 'https://images.unsplash.com/photo-1606980631035-2e57ec15c0c4?w=800'),
        (uuid_generate_v4(), 'Sony A7 IV', 'sony-a7-iv', 'Versatile hybrid camera with 33MP sensor, 4K 60p video, and 10-bit recording', 2299.99, 2498.99, 10, cameras_id, 'SNY-A74-BDY', true, 'https://images.unsplash.com/photo-1606983340126-99ab4feaa64a?w=800'),
        (uuid_generate_v4(), 'Nikon Z6 III', 'nikon-z6-iii', 'Professional mirrorless with in-body stabilization and dual card slots', 2199.99, 2399.99, 8, cameras_id, 'NIK-Z6M3-BDY', true, 'https://images.unsplash.com/photo-1606982468106-8771b8feb2d8?w=800'),
        (uuid_generate_v4(), 'Fujifilm X-T5', 'fujifilm-xt5', 'Retro-styled APS-C camera with 40MP sensor and film simulations', 1599.99, 1699.99, 15, cameras_id, 'FUJ-XT5-SLV', true, 'https://images.unsplash.com/photo-1606982419632-48a4f3108814?w=800');

    -- Smartwatches
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Apple Watch Ultra 2', 'apple-watch-ultra-2', 'Rugged titanium smartwatch with action button and dual-frequency GPS', 799.99, null, 42, wearables_id, 'APL-AWU2-TIT', true, 'https://images.unsplash.com/photo-1579586337278-3befd40fd17a?w=800'),
        (uuid_generate_v4(), 'Samsung Galaxy Watch 6 Classic', 'galaxy-watch-6-classic', 'Premium smartwatch with rotating bezel and comprehensive health tracking', 379.99, 429.99, 58, wearables_id, 'SAM-GW6C-BLK', true, 'https://images.unsplash.com/photo-1579721840641-7d0e67f1204e?w=800'),
        (uuid_generate_v4(), 'Garmin Fenix 7X Solar', 'garmin-fenix-7x-solar', 'Multisport GPS watch with solar charging and advanced training metrics', 899.99, 949.99, 25, wearables_id, 'GAR-F7X-SOL', true, 'https://images.unsplash.com/photo-1557438159-51eec7a6c9e8?w=800'),
        (uuid_generate_v4(), 'Fitbit Sense 2', 'fitbit-sense-2', 'Health-focused smartwatch with stress management and ECG', 249.99, 299.99, 65, wearables_id, 'FTB-SEN2-GRY', true, 'https://images.unsplash.com/photo-1575311373937-040b8e1fd5b6?w=800');

    -- FURNITURE PRODUCTS
    
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Modern L-Shaped Sectional Sofa', 'modern-l-shaped-sectional', 'Contemporary grey fabric sectional with reversible chaise and accent pillows', 1299.99, 1599.99, 8, furniture_id, 'FRN-SEC-GRY', true, 'https://images.unsplash.com/photo-1555041469-a586c61ea9bc?w=800'),
        (uuid_generate_v4(), 'Industrial Dining Table Set', 'industrial-dining-set', 'Rustic wood and metal dining table with 6 chairs, seats 8 comfortably', 899.99, 1099.99, 12, furniture_id, 'FRN-DIN-IND', true, 'https://images.unsplash.com/photo-1617806118233-18e1de247200?w=800'),
        (uuid_generate_v4(), 'Platform Bed Frame with Storage', 'platform-bed-storage', 'Queen size upholstered bed with hydraulic lift storage and USB charging', 649.99, 799.99, 15, furniture_id, 'FRN-BED-QPL', true, 'https://images.unsplash.com/photo-1505693416388-ac5ce068fe85?w=800'),
        (uuid_generate_v4(), 'Executive Office Desk', 'executive-office-desk', 'Solid wood executive desk with file drawers and cable management', 749.99, 899.99, 10, furniture_id, 'FRN-DSK-EXE', true, 'https://images.unsplash.com/photo-1518455027359-f3f8164ba6bd?w=800'),
        (uuid_generate_v4(), 'Ergonomic Gaming Chair', 'ergonomic-gaming-chair', 'Premium racing-style chair with lumbar support, adjustable armrests, and reclining', 329.99, 399.99, 28, furniture_id, 'FRN-CHR-GMG', true, 'https://images.unsplash.com/photo-1580480055273-228ff5388ef8?w=800'),
        (uuid_generate_v4(), 'Mid-Century Bookshelf', 'mid-century-bookshelf', 'Walnut finish open bookcase with 5 shelves, perfect for living room or office', 279.99, 349.99, 20, furniture_id, 'FRN-BKS-MCM', true, 'https://images.unsplash.com/photo-1594620302200-9a762244a156?w=800'),
        (uuid_generate_v4(), 'Velvet Accent Chair', 'velvet-accent-chair', 'Luxurious emerald green velvet armchair with gold metal legs', 399.99, 499.99, 18, furniture_id, 'FRN-ACC-VEL', true, 'https://images.unsplash.com/photo-1586023492125-27b2c045efd7?w=800'),
        (uuid_generate_v4(), 'Glass Coffee Table', 'glass-coffee-table', 'Modern tempered glass top coffee table with chrome base and storage shelf', 249.99, null, 22, furniture_id, 'FRN-CFT-GLS', true, 'https://images.unsplash.com/photo-1565191999001-551c187427bb?w=800');

    -- Kitchen & Dining
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Stainless Steel Cookware Set', 'cookware-set-12pc', 'Professional 12-piece stainless steel cookware with aluminum core', 299.99, 399.99, 45, kitchen_id, 'KIT-CKW-12PC', true, 'https://images.unsplash.com/photo-1584990347449-39b4aa1d40b4?w=800'),
        (uuid_generate_v4(), 'KitchenAid Stand Mixer', 'kitchenaid-stand-mixer', 'Iconic 5-quart tilt-head stand mixer with 10 speeds, multiple colors available', 379.99, 449.99, 32, kitchen_id, 'KIT-MIX-5QT', true, 'https://images.unsplash.com/photo-1578916171728-46686eac8d58?w=800'),
        (uuid_generate_v4(), 'Ninja Air Fryer XL', 'ninja-air-fryer-xl', 'Extra large air fryer with 8 cooking functions and crisper basket', 119.99, 149.99, 68, kitchen_id, 'KIT-AFR-XL', true, 'https://images.unsplash.com/photo-1585515320310-259814833e62?w=800'),
        (uuid_generate_v4(), 'Dinnerware Set 16-Piece', 'dinnerware-set-modern', 'Modern porcelain dinnerware set, service for 4, dishwasher safe', 89.99, 119.99, 52, kitchen_id, 'KIT-DIN-16PC', true, 'https://images.unsplash.com/photo-1610701596007-11502861dcfa?w=800');

    -- Home Decor
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Abstract Canvas Wall Art Set', 'abstract-canvas-3pc', 'Modern 3-piece abstract canvas prints, framed and ready to hang', 149.99, 199.99, 35, decor_id, 'DEC-ART-ABS3', true, 'https://images.unsplash.com/photo-1513519245088-0e12902e5a38?w=800'),
        (uuid_generate_v4(), 'Bohemian Area Rug 8x10', 'bohemian-rug-8x10', 'Vintage-inspired distressed area rug with geometric patterns', 189.99, 249.99, 18, decor_id, 'DEC-RUG-BOH8', true, 'https://images.unsplash.com/photo-1600166898405-da9535204843?w=800'),
        (uuid_generate_v4(), 'Floor Mirror Full Length', 'floor-mirror-gold', 'Arched full-length standing mirror with gold metal frame', 179.99, 229.99, 24, decor_id, 'DEC-MIR-FLR', true, 'https://images.unsplash.com/photo-1618220179428-22790b461013?w=800'),
        (uuid_generate_v4(), 'Luxury Scented Candle Set', 'luxury-candle-set', 'Premium soy candles in ceramic vessels, set of 3 seasonal scents', 59.99, 79.99, 88, decor_id, 'DEC-CND-LUX3', true, 'https://images.unsplash.com/photo-1602874801006-39554fcbeadc?w=800');

    -- EXERCISE & FITNESS PRODUCTS
    
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Adjustable Dumbbell Set', 'adjustable-dumbbell-set', 'Space-saving dumbbells adjustable from 5-52.5 lbs with quick-change dial', 349.99, 449.99, 42, exercise_id, 'FIT-DUM-ADJ52', true, 'https://images.unsplash.com/photo-1638536532686-d610adfc8e5c?w=800'),
        (uuid_generate_v4(), 'Yoga Mat Premium', 'yoga-mat-premium', 'Extra thick eco-friendly TPE yoga mat with alignment marks and carrying strap', 39.99, 59.99, 125, exercise_id, 'FIT-YOG-PREM', true, 'https://images.unsplash.com/photo-1601925260368-ae2f83cf8b7f?w=800'),
        (uuid_generate_v4(), 'Resistance Bands Set', 'resistance-bands-set', 'Complete set of 5 resistance bands with handles, door anchor, and carry bag', 29.99, 39.99, 158, exercise_id, 'FIT-RES-SET5', true, 'https://images.unsplash.com/photo-1598289431512-b97b0917affc?w=800'),
        (uuid_generate_v4(), 'Treadmill Folding Electric', 'treadmill-folding', 'Compact folding treadmill with 12 preset programs and tablet holder', 599.99, 799.99, 15, exercise_id, 'FIT-TRD-FLD', true, 'https://images.unsplash.com/photo-1576678927484-cc907957fb50?w=800'),
        (uuid_generate_v4(), 'Kettlebell Set 3-Piece', 'kettlebell-set-3pc', 'Cast iron kettlebells - 15, 25, 35 lbs with wide handles', 119.99, 149.99, 38, exercise_id, 'FIT-KTL-3PC', true, 'https://images.unsplash.com/photo-1623874106933-ab11e15478c4?w=800'),
        (uuid_generate_v4(), 'Exercise Bike Stationary', 'exercise-bike-stationary', 'Indoor cycling bike with magnetic resistance and digital monitor', 399.99, 499.99, 22, exercise_id, 'FIT-BIK-STAT', true, 'https://images.unsplash.com/photo-1556817411-58c45dd94e8c?w=800'),
        (uuid_generate_v4(), 'Pull-Up Bar Doorway', 'pull-up-bar-doorway', 'Heavy-duty doorway pull-up bar with multiple grip positions, no screws needed', 34.99, 44.99, 95, exercise_id, 'FIT-PUL-DOOR', true, 'https://images.unsplash.com/photo-1584464491033-06628f3a6b7b?w=800'),
        (uuid_generate_v4(), 'Foam Roller Massage', 'foam-roller-massage', 'High-density foam roller for muscle recovery and myofascial release', 24.99, null, 142, exercise_id, 'FIT-FOM-ROL', true, 'https://images.unsplash.com/photo-1611933021975-ea27bab5e179?w=800'),
        (uuid_generate_v4(), 'Weight Bench Adjustable', 'weight-bench-adjustable', 'Multi-position weight bench with leg developer and preacher curl pad', 229.99, 299.99, 18, exercise_id, 'FIT-BNC-ADJ', true, 'https://images.unsplash.com/photo-1517836357463-d25dfeac3438?w=800'),
        (uuid_generate_v4(), 'Jump Rope Speed', 'jump-rope-speed', 'Tangle-free ball bearing jump rope with adjustable length and comfortable handles', 14.99, 19.99, 200, exercise_id, 'FIT-JMP-SPD', true, 'https://images.unsplash.com/photo-1601422407692-ec4eeec1d9b3?w=800');

    -- Outdoor Recreation
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Camping Tent 4-Person', 'camping-tent-4person', 'Waterproof dome tent with easy setup, ventilation windows, and carry bag', 149.99, 199.99, 28, outdoor_id, 'OUT-TNT-4P', true, 'https://images.unsplash.com/photo-1504280390367-361c6d9f38f4?w=800'),
        (uuid_generate_v4(), 'Hiking Backpack 50L', 'hiking-backpack-50l', 'Ergonomic hiking backpack with rain cover and multiple compartments', 89.99, 119.99, 45, outdoor_id, 'OUT-BPK-50L', true, 'https://images.unsplash.com/photo-1622260614153-03223fb72052?w=800'),
        (uuid_generate_v4(), 'Sleeping Bag Mummy', 'sleeping-bag-mummy', '3-season mummy sleeping bag rated to 20°F with compression sack', 79.99, 99.99, 52, outdoor_id, 'OUT-SLP-MMY', true, 'https://images.unsplash.com/photo-1609198092357-efca2f91b609?w=800'),
        (uuid_generate_v4(), 'Portable Camping Stove', 'camping-stove-portable', 'Compact propane camping stove with windshield and piezo ignition', 44.99, 59.99, 68, outdoor_id, 'OUT-STV-PRT', true, 'https://images.unsplash.com/photo-1613588729903-471bc2652ca9?w=800');

    -- Cycling
    INSERT INTO products (id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url) VALUES
        (uuid_generate_v4(), 'Mountain Bike 29er', 'mountain-bike-29er', 'Full suspension mountain bike with 21-speed Shimano drivetrain', 799.99, 999.99, 12, cycling_id, 'CYC-MTB-29', true, 'https://images.unsplash.com/photo-1576435728678-68d0fbf94e91?w=800'),
        (uuid_generate_v4(), 'Road Bike Carbon Fiber', 'road-bike-carbon', 'Lightweight carbon fiber road bike with drop bars and 18-speed gearing', 1499.99, 1799.99, 8, cycling_id, 'CYC-RDB-CF', true, 'https://images.unsplash.com/photo-1485965120184-e220f721d03e?w=800'),
        (uuid_generate_v4(), 'Bike Helmet Adult', 'bike-helmet-adult', 'Aerodynamic cycling helmet with MIPS technology and adjustable fit', 69.99, 89.99, 85, cycling_id, 'CYC-HLM-MPS', true, 'https://images.unsplash.com/photo-1557689560-72a512d3a9c0?w=800'),
        (uuid_generate_v4(), 'Cycling Shorts Padded', 'cycling-shorts-padded', 'Professional cycling shorts with gel padding and moisture-wicking fabric', 49.99, 64.99, 110, cycling_id, 'CYC-SHT-PAD', true, 'https://images.unsplash.com/photo-1559827260-dc66d52bef19?w=800');

END $$;