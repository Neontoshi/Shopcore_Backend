use sqlx::PgPool;
use uuid::Uuid;
// use crate::models::ReviewReply;
use crate::repositories::ReviewRepository;
use crate::dtos::review_dto::{CreateReviewRequest, ProductRatingSummary, ReviewResponse, ReplyResponse};
use crate::errors::AppError;

pub struct ReviewService;

impl ReviewService {
    pub async fn create_review(
        pool: &PgPool,
        user_id: &Uuid,
        req: CreateReviewRequest,
    ) -> Result<ReviewResponse, AppError> {
        let product_id = req.product_id.ok_or_else(|| AppError::bad_request("Product ID is required"))?;
        
        let review = ReviewRepository::create_review(
            pool,
            user_id,
            &product_id,
            req.rating,
            req.title,
            req.comment,
        ).await?;
        
        Ok(review.into())
    }
    
    pub async fn get_product_reviews(
        pool: &PgPool,
        product_id: &Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<ReviewResponse>, i64, ProductRatingSummary), AppError> {
        let (reviews, total) = ReviewRepository::get_product_reviews(pool, product_id, page, page_size).await?;
        let summary = ReviewRepository::get_product_rating_summary(pool, product_id).await?;
        
        let mut responses: Vec<ReviewResponse> = Vec::new();
        for review in reviews {
            let mut response: ReviewResponse = review.into();
            // Get user name
            let user = sqlx::query!(
                r#"
                SELECT first_name, last_name FROM users WHERE id = $1
                "#,
                response.user_id
            )
            .fetch_optional(pool)
            .await?;
            
            if let Some(user) = user {
                let first = user.first_name.unwrap_or_default();
                let last = user.last_name.unwrap_or_default();
                let name = if first.is_empty() && last.is_empty() {
                    "Anonymous".to_string()
                } else {
                    format!("{} {}", first, last).trim().to_string()
                };
                response.user_name = Some(name);
            }
            responses.push(response);
        }
        
        Ok((responses, total, summary))
    }
    
    pub async fn mark_helpful(
        pool: &PgPool,
        review_id: &Uuid,
        is_helpful: bool,
    ) -> Result<(), AppError> {
        ReviewRepository::mark_helpful(pool, review_id, is_helpful).await?;
        Ok(())
    }
    
    pub async fn add_reply(
        pool: &PgPool,
        review_id: &Uuid,
        user_id: &Uuid,
        reply: &str,
    ) -> Result<ReplyResponse, AppError> {
        let reply_record = ReviewRepository::add_reply(pool, review_id, user_id, reply).await?;
        
        let mut response = ReplyResponse {
            id: reply_record.id,
            review_id: reply_record.review_id,
            user_id: reply_record.user_id,
            reply: reply_record.reply,
            created_at: reply_record.created_at,
            user_name: None,
        };
        
        let user = sqlx::query!(
            r#"
            SELECT first_name, last_name FROM users WHERE id = $1
            "#,
            response.user_id
        )
        .fetch_optional(pool)
        .await?;
        
        if let Some(user) = user {
            let first = user.first_name.unwrap_or_default();
            let last = user.last_name.unwrap_or_default();
            response.user_name = Some(format!("{} {}", first, last).trim().to_string());
        }
        
        Ok(response)
    }
    
    pub async fn get_review_replies(
        pool: &PgPool,
        review_id: &Uuid,
    ) -> Result<Vec<ReplyResponse>, AppError> {
        let replies = ReviewRepository::get_review_replies(pool, review_id).await?;
        
        let mut responses = Vec::new();
        for reply in replies {
            let mut response = ReplyResponse {
                id: reply.id,
                review_id: reply.review_id,
                user_id: reply.user_id,
                reply: reply.reply,
                created_at: reply.created_at,
                user_name: None,
            };
            
            let user = sqlx::query!(
                r#"
                SELECT first_name, last_name FROM users WHERE id = $1
                "#,
                response.user_id
            )
            .fetch_optional(pool)
            .await?;
            
            if let Some(user) = user {
                let first = user.first_name.unwrap_or_default();
                let last = user.last_name.unwrap_or_default();
                response.user_name = Some(format!("{} {}", first, last).trim().to_string());
            }
            
            responses.push(response);
        }
        
        Ok(responses)
    }
}
