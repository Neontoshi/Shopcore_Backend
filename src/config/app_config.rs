use serde::Deserialize;
use dotenvy::dotenv;
use anyhow::Context;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub app_version: String,
    pub app_env: String,
    pub app_port: u16,
    pub app_host: String,
    
    pub database_url: String,
    pub redis_url: String,
    
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub bcrypt_cost: u32,

    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_public_key: String,
    
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    
    pub admin_email: String,  // ADD THIS
    
    pub rate_limit_requests: u32,
    pub rate_limit_duration_seconds: u64,
    
    pub default_page_size: usize,
    pub max_page_size: usize,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        
        Ok(Self {
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "Shopcore API".to_string()),
            app_version: std::env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
            app_env: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            app_port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("Invalid APP_PORT")?,
            app_host: std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            
            stripe_secret_key: std::env::var("STRIPE_SECRET_KEY")
                .context("STRIPE_SECRET_KEY must be set")?,
            stripe_webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET")
                .context("STRIPE_WEBHOOK_SECRET must be set")?,
            stripe_public_key: std::env::var("STRIPE_PUBLIC_KEY")
                .context("STRIPE_PUBLIC_KEY must be set")?,
            
            smtp_host: std::env::var("SMTP_HOST")
                .unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_username: std::env::var("SMTP_USERNAME")
                .context("SMTP_USERNAME must be set")?,
            smtp_password: std::env::var("SMTP_PASSWORD")
                .context("SMTP_PASSWORD must be set")?,
            smtp_from: std::env::var("SMTP_FROM")
                .context("SMTP_FROM must be set")?,
            
            admin_email: std::env::var("ADMIN_EMAIL")
                .unwrap_or_else(|_| "daisisamuel23@gmail.com".to_string()),  // ADD THIS
            jwt_secret: std::env::var("JWT_SECRET")
                .context("JWT_SECRET must be set")?,
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .context("Invalid JWT_EXPIRATION_HOURS")?,
            bcrypt_cost: std::env::var("BCRYPT_COST")
                .unwrap_or_else(|_| "12".to_string())
                .parse()
                .context("Invalid BCRYPT_COST")?,
            
            rate_limit_requests: std::env::var("RATE_LIMIT_REQUESTS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_REQUESTS")?,
            rate_limit_duration_seconds: std::env::var("RATE_LIMIT_DURATION_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_DURATION_SECONDS")?,
            
            default_page_size: std::env::var("DEFAULT_PAGE_SIZE")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .context("Invalid DEFAULT_PAGE_SIZE")?,
            max_page_size: std::env::var("MAX_PAGE_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .context("Invalid MAX_PAGE_SIZE")?,
        })
    }
    
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }
}