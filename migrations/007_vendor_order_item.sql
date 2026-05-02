-- Add vendor_id to order_items for frozen attribution at purchase time
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS vendor_id UUID REFERENCES users(id);

-- Backfill from products table
UPDATE order_items oi
SET vendor_id = p.vendor_id
FROM products p
WHERE oi.product_id = p.id
AND p.vendor_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_order_items_vendor_id ON order_items(vendor_id);