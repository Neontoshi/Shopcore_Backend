use std::sync::Arc;
use sqlx::PgPool;
use crate::config::AppConfig;
use crate::utils::JwtService;
use crate::services::EmailService;
use crate::config::redis::RedisClient;
use crate::utils::cache::cache_all_products;
use crate::repositories::ProductRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: PgPool,
    pub jwt_service: Arc<JwtService>,
    pub email_service: Arc<EmailService>,
    pub redis_client: RedisClient,
}

impl AppState {
    pub async fn new(config: AppConfig, db_pool: PgPool) -> Result<Self, crate::errors::AppError> {
        tracing::info!("Initializing AppState...");
        
        let jwt_service = JwtService::new(
            &config.jwt_secret,
            config.jwt_expiration_hours,
        );

        let email_service = EmailService::new(&config)?;
        
        tracing::info!("Connecting to Redis...");
        let redis_client = RedisClient::new(&config).await?;
        tracing::info!("Redis connected successfully");
        
        tracing::info!("Fetching products from database...");
        match ProductRepository::find_all(&db_pool).await {
            Ok(products) => {
            let products: Vec<crate::models::Product> = products;
                tracing::info!("Found {} products in database", products.len());
                if !products.is_empty() {
                    tracing::info!("Caching products in Redis...");
                    if let Err(e) = cache_all_products(&redis_client, &products).await {
                        tracing::warn!("Failed to cache products on startup: {}", e);
                    } else {
                        tracing::info!("Cached {} products in Redis on startup", products.len());
                    }
                } else {
                    tracing::warn!("No products found to cache");
                }
            }
            Err(e) => {
                tracing::warn!("Failed to fetch products for caching: {}", e);
            }
        }

        Ok(Self {
            config: Arc::new(config),
            db_pool,
            jwt_service: Arc::new(jwt_service),
            email_service: Arc::new(email_service),
            redis_client,
        })
    }

    pub fn get_db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_jwt_service(&self) -> &JwtService {
        &self.jwt_service
    }

    pub fn get_email_service(&self) -> &EmailService {
        &self.email_service
    }
}