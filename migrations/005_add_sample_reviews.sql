-- Add sample reviews for existing products
DO $$
DECLARE
    user_record RECORD;
    product_record RECORD;
BEGIN
    -- Check if reviews already exist
    IF (SELECT COUNT(*) FROM reviews) = 0 THEN
        -- Add reviews for random products
        FOR product_record IN 
            SELECT id FROM products LIMIT 20
        LOOP
            FOR user_record IN 
                SELECT id FROM users WHERE role = 'customer' LIMIT 3
            LOOP
                INSERT INTO reviews (
                    user_id, 
                    product_id, 
                    rating, 
                    title, 
                    comment, 
                    is_approved,
                    is_verified_purchase
                ) VALUES (
                    user_record.id,
                    product_record.id,
                    (random() * 4 + 1)::INTEGER,
                    CASE (random() * 3)::INTEGER
                        WHEN 0 THEN 'Excellent product!'
                        WHEN 1 THEN 'Good value'
                        ELSE 'Satisfied with purchase'
                    END,
                    CASE (random() * 3)::INTEGER
                        WHEN 0 THEN 'Really happy with this product. Quality is great and delivery was fast.'
                        WHEN 1 THEN 'Good product for the price. Would recommend to others.'
                        ELSE 'Meets expectations. No issues so far.'
                    END,
                    true,
                    true
                );
            END LOOP;
        END LOOP;
        
        -- Update product ratings
        REFRESH MATERIALIZED VIEW CONCURRENTLY product_search_view;
        
        RAISE NOTICE 'Sample reviews added successfully!';
    ELSE
        RAISE NOTICE 'Reviews already exist, skipping sample data.';
    END IF;
END $$;
