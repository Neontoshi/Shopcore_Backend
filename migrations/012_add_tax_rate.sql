-- Add tax_rate column to platform_settings
ALTER TABLE platform_settings ADD COLUMN IF NOT EXISTS tax_rate DECIMAL(5,2) NOT NULL DEFAULT 7.50;

-- Update all existing rows to have the default tax rate
UPDATE platform_settings SET tax_rate = 7.50 WHERE tax_rate IS NULL OR tax_rate = 0;

-- Also update the initial insert from migration 011 to include tax_rate
-- (If the table is empty, insert a default row)
INSERT INTO platform_settings (platform_fee_percent, tax_rate)
SELECT 2.80, 7.50
WHERE NOT EXISTS (SELECT 1 FROM platform_settings WHERE is_active = true);
