-- Add weight column to products
ALTER TABLE products ADD COLUMN IF NOT EXISTS weight DECIMAL(10,2) DEFAULT 0;

-- Update weights for existing products
UPDATE products SET weight = 
    CASE 
        -- Smartphones & Tablets (0.2 - 0.7 kg)
        WHEN name ILIKE '%iPhone%' THEN 0.22
        WHEN name ILIKE '%Samsung Galaxy%' THEN 0.23
        WHEN name ILIKE '%iPad%' THEN 0.68
        WHEN name ILIKE '%Google Pixel%' THEN 0.21
        WHEN name ILIKE '%OnePlus%' THEN 0.22
        
        -- Laptops (1.5 - 3.5 kg)
        WHEN name ILIKE '%ROG Strix%' THEN 2.5
        WHEN name ILIKE '%MSI Katana%' THEN 2.3
        WHEN name ILIKE '%Razer Blade%' THEN 2.1
        WHEN name ILIKE '%Legion Pro%' THEN 2.8
        WHEN name ILIKE '%Predator Helios%' THEN 2.6
        WHEN name ILIKE '%Alienware%' THEN 3.2
        
        -- Headphones & Earbuds (0.2 - 0.4 kg)
        WHEN name ILIKE '%WH-1000XM5%' THEN 0.25
        WHEN name ILIKE '%QuietComfort Ultra%' THEN 0.29
        WHEN name ILIKE '%AirPods Max%' THEN 0.39
        WHEN name ILIKE '%Momentum 4%' THEN 0.29
        WHEN name ILIKE '%AirPods Pro%' THEN 0.05
        WHEN name ILIKE '%Galaxy Buds%' THEN 0.05
        WHEN name ILIKE '%WF-1000XM5%' THEN 0.05
        WHEN name ILIKE '%Jabra Elite%' THEN 0.05
        
        -- Cameras (0.5 - 1.5 kg)
        WHEN name ILIKE '%Canon EOS%' THEN 1.2
        WHEN name ILIKE '%Sony A7%' THEN 1.1
        WHEN name ILIKE '%Nikon Z6%' THEN 1.1
        WHEN name ILIKE '%Fujifilm X-T5%' THEN 0.9
        
        -- Smartwatches (0.05 - 0.1 kg)
        WHEN name ILIKE '%Apple Watch%' THEN 0.06
        WHEN name ILIKE '%Galaxy Watch%' THEN 0.06
        WHEN name ILIKE '%Garmin Fenix%' THEN 0.08
        WHEN name ILIKE '%Fitbit Sense%' THEN 0.05
        
        -- Furniture
        WHEN name ILIKE '%Sectional Sofa%' THEN 45.0
        WHEN name ILIKE '%Dining Table Set%' THEN 35.0
        WHEN name ILIKE '%Platform Bed%' THEN 40.0
        WHEN name ILIKE '%Executive Desk%' THEN 25.0
        WHEN name ILIKE '%Gaming Chair%' THEN 18.0
        WHEN name ILIKE '%Bookshelf%' THEN 15.0
        WHEN name ILIKE '%Accent Chair%' THEN 12.0
        WHEN name ILIKE '%Coffee Table%' THEN 10.0
        
        -- Kitchen
        WHEN name ILIKE '%Cookware Set%' THEN 5.0
        WHEN name ILIKE '%KitchenAid%' THEN 8.0
        WHEN name ILIKE '%Air Fryer%' THEN 4.5
        WHEN name ILIKE '%Dinnerware Set%' THEN 3.0
        
        -- Home Decor
        WHEN name ILIKE '%Wall Art%' THEN 2.0
        WHEN name ILIKE '%Area Rug%' THEN 4.0
        WHEN name ILIKE '%Floor Mirror%' THEN 8.0
        WHEN name ILIKE '%Candle Set%' THEN 0.8
        
        -- Exercise Equipment
        WHEN name ILIKE '%Dumbbell Set%' THEN 18.0
        WHEN name ILIKE '%Yoga Mat%' THEN 0.8
        WHEN name ILIKE '%Resistance Bands%' THEN 0.3
        WHEN name ILIKE '%Treadmill%' THEN 55.0
        WHEN name ILIKE '%Kettlebell%' THEN 12.0
        WHEN name ILIKE '%Exercise Bike%' THEN 35.0
        WHEN name ILIKE '%Pull-Up Bar%' THEN 2.5
        WHEN name ILIKE '%Foam Roller%' THEN 0.6
        WHEN name ILIKE '%Weight Bench%' THEN 22.0
        WHEN name ILIKE '%Jump Rope%' THEN 0.2
        
        -- Outdoor
        WHEN name ILIKE '%Camping Tent%' THEN 3.5
        WHEN name ILIKE '%Hiking Backpack%' THEN 1.2
        WHEN name ILIKE '%Sleeping Bag%' THEN 1.5
        WHEN name ILIKE '%Camping Stove%' THEN 0.8
        
        -- Cycling
        WHEN name ILIKE '%Mountain Bike%' THEN 14.0
        WHEN name ILIKE '%Road Bike%' THEN 8.5
        WHEN name ILIKE '%Bike Helmet%' THEN 0.3
        WHEN name ILIKE '%Cycling Shorts%' THEN 0.2
        
        ELSE 0.5
    END
WHERE weight IS NULL OR weight = 0;

-- Verify weights were added
SELECT 'Products with weights assigned:' as info, COUNT(*) as count FROM products WHERE weight > 0;
SELECT 'Average weight:' as info, AVG(weight) as avg_kg FROM products;
