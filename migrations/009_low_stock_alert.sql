-- Low stock alerts tracking table
CREATE TABLE IF NOT EXISTS low_stock_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    alert_type VARCHAR(50) NOT NULL, -- 'admin', 'vendor'
    sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    stock_at_alert INTEGER NOT NULL,
    threshold INTEGER NOT NULL DEFAULT 5,
    UNIQUE(product_id, alert_type)
);

-- Index for faster lookups
CREATE INDEX idx_low_stock_alerts_product ON low_stock_alerts(product_id);
CREATE INDEX idx_low_stock_alerts_sent_at ON low_stock_alerts(sent_at);

-- Add threshold column to products (if not exists)
ALTER TABLE products ADD COLUMN IF NOT EXISTS low_stock_threshold INTEGER DEFAULT 5;

-- Add last_alert_sent_at column to prevent spamming
ALTER TABLE products ADD COLUMN IF NOT EXISTS last_low_stock_alert_sent_at TIMESTAMP WITH TIME ZONE;