-- Platform fee settings
CREATE TABLE platform_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    platform_fee_percent DECIMAL(5,2) NOT NULL DEFAULT 2.80,
    is_active BOOLEAN NOT NULL DEFAULT true,
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default
INSERT INTO platform_settings (platform_fee_percent) VALUES (2.80);
