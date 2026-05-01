use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Review {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub is_verified_purchase: bool,
    pub helpful_count: i32,
    pub unhelpful_count: i32,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReviewReply {
    pub id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub reply: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
