// App-wide constants
pub const APP_NAME: &str = "Shopcore API";
pub const APP_VERSION: &str = "0.1.0";
pub const DEFAULT_PAGE_SIZE: usize = 20;
pub const MAX_PAGE_SIZE: usize = 100;
pub const MAX_PAGE_LIMIT: usize = 100;

// Validation constants
pub const MAX_PRODUCT_NAME_LENGTH: usize = 255;
pub const MAX_PRODUCT_DESCRIPTION_LENGTH: usize = 5000;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 72;
pub const MAX_EMAIL_LENGTH: usize = 255;
pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_REVIEW_COMMENT_LENGTH: usize = 1000;

// Price constants
pub const MIN_PRICE: f64 = 0.0;
pub const MAX_PRICE: f64 = 999999.99;
pub const MIN_STOCK_QUANTITY: i32 = 0;
pub const MAX_STOCK_QUANTITY: i32 = 999999;

// Cache constants (seconds)
pub const CACHE_DURATION_PRODUCT: u64 = 300; // 5 minutes
pub const CACHE_DURATION_CATEGORY: u64 = 600; // 10 minutes
pub const CACHE_DURATION_USER: u64 = 60; // 1 minute