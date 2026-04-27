use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};
use anyhow::Context;
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub role: String,
    pub exp: usize,   // Expiration timestamp
    pub iat: usize,   // Issued at timestamp
    pub jti: String,  // JWT ID (for refresh tokens)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub jti: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_hours: i64,
}

impl JwtService {
    pub fn new(secret: &str, expiration_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiration_hours,
        }
    }
    
    pub fn generate_access_token(&self, user_id: &Uuid, email: &str, role: &str) -> anyhow::Result<String> {
        let now = Utc::now();
        let expire = now + Duration::hours(self.expiration_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: expire.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .context("Failed to generate access token")
    }
    
    pub fn generate_refresh_token(&self, user_id: &Uuid) -> anyhow::Result<String> {
        let now = Utc::now();
        let expire = now + Duration::days(30); // Refresh tokens last 30 days
        
        let claims = RefreshTokenClaims {
            sub: user_id.to_string(),
            jti: Uuid::new_v4().to_string(),
            exp: expire.timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .context("Failed to generate refresh token")
    }
    
    pub fn verify_access_token(&self, token: &str) -> anyhow::Result<TokenData<Claims>> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.decoding_key, &validation)
            .context("Invalid or expired access token")
    }
    
    pub fn verify_refresh_token(&self, token: &str) -> anyhow::Result<TokenData<RefreshTokenClaims>> {
        let validation = Validation::default();
        decode::<RefreshTokenClaims>(token, &self.decoding_key, &validation)
            .context("Invalid or expired refresh token")
    }
    
    pub fn extract_user_id_from_token(&self, token: &str) -> anyhow::Result<Uuid> {
        let token_data = self.verify_access_token(token)?;
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .context("Invalid user ID in token")?;
        Ok(user_id)
    }
}

