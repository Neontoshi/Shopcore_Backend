use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::Review;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewRequest {
    #[validate(required)]
    pub product_id: Option<Uuid>,
    
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    
    #[validate(length(max = 255))]
    pub title: Option<String>,
    
    #[validate(length(max = 2000))]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateReviewRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: Option<i32>,
    
    #[validate(length(max = 255))]
    pub title: Option<String>,
    
    #[validate(length(max = 2000))]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReplyRequest {
    #[validate(length(min = 1, max = 1000))]
    pub reply: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
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
    pub user_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReplyResponse {
    pub id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub reply: String,
    pub created_at: DateTime<Utc>,
    pub user_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductRatingSummary {
    pub average_rating: f64,
    pub total_reviews: i64,
    pub rating_5_stars: i64,
    pub rating_4_stars: i64,
    pub rating_3_stars: i64,
    pub rating_2_stars: i64,
    pub rating_1_stars: i64,
}

impl From<Review> for ReviewResponse {
    fn from(review: Review) -> Self {
        ReviewResponse {
            id: review.id,
            user_id: review.user_id,
            product_id: review.product_id,
            rating: review.rating,
            title: review.title,
            comment: review.comment,
            is_verified_purchase: review.is_verified_purchase,
            helpful_count: review.helpful_count,
            unhelpful_count: review.unhelpful_count,
            is_approved: review.is_approved,
            created_at: review.created_at,
            user_name: None,
        }
    }
}
