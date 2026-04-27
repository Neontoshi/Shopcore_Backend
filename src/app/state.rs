use std::sync::Arc;
use sqlx::PgPool;
use crate::config::AppConfig;
use crate::utils::JwtService;
use crate::services::EmailService;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: PgPool,
    pub jwt_service: Arc<JwtService>,
    pub email_service: Arc<EmailService>,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: PgPool) -> Result<Self, crate::errors::AppError> {
        let jwt_service = JwtService::new(
            &config.jwt_secret,
            config.jwt_expiration_hours,
        );

        let email_service = EmailService::new(&config)?;

        Ok(Self {
            config: Arc::new(config),
            db_pool,
            jwt_service: Arc::new(jwt_service),
            email_service: Arc::new(email_service),
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