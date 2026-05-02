use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Review, ReviewReply};
use crate::errors::AppError;

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn create_review(
        pool: &PgPool,
        user_id: &Uuid,
        product_id: &Uuid,
        rating: i32,
        title: Option<String>,
        comment: Option<String>,
    ) -> Result<Review, AppError> {
        let review = sqlx::query_as!(
            Review,
            r#"
            INSERT INTO reviews (user_id, product_id, rating, title, comment, is_approved)
            VALUES ($1, $2, $3, $4, $5, true)
            RETURNING id, user_id, product_id, rating, title, comment, 
                      is_verified_purchase, helpful_count, unhelpful_count, 
                      is_approved, created_at, updated_at, deleted_at
            "#,
            user_id,
            product_id,
            rating,
            title,
            comment
        )
        .fetch_one(pool)
        .await?;
        
        Ok(review)
    }
    
    pub async fn get_product_reviews(
        pool: &PgPool,
        product_id: &Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<Review>, i64), AppError> {
        let offset = (page - 1) * page_size;
        
        let reviews = sqlx::query_as!(
            Review,
            r#"
            SELECT r.id, r.user_id, r.product_id, r.rating, r.title, r.comment,
                   r.is_verified_purchase, r.helpful_count, r.unhelpful_count,
                   r.is_approved, r.created_at, r.updated_at, r.deleted_at
            FROM reviews r
            WHERE r.product_id = $1 AND r.is_approved = true AND r.deleted_at IS NULL
            ORDER BY r.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            product_id,
            page_size,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reviews
            WHERE product_id = $1 AND is_approved = true AND deleted_at IS NULL
            "#,
            product_id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);
        
        Ok((reviews, total))
    }
    
    pub async fn mark_helpful(
    pool: &PgPool,
    review_id: &Uuid,
    user_id: &Uuid,
    is_helpful: bool,
) -> Result<(), AppError> {
    // First, check if user already voted
    let existing = sqlx::query!(
        r#"
        SELECT id FROM review_helpfulness 
        WHERE review_id = $1 AND user_id = $2
        "#,
        review_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(AppError::bad_request("You have already voted on this review"));
    }

    // Insert the vote record
    sqlx::query!(
        r#"
        INSERT INTO review_helpfulness (id, review_id, user_id, is_helpful)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        review_id,
        user_id,
        is_helpful
    )
    .execute(pool)
    .await?;

    // Update the review counts
    if is_helpful {
        sqlx::query!(
            r#"
            UPDATE reviews SET helpful_count = helpful_count + 1, updated_at = NOW()
            WHERE id = $1
            "#,
            review_id
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
            UPDATE reviews SET unhelpful_count = unhelpful_count + 1, updated_at = NOW()
            WHERE id = $1
            "#,
            review_id
        )
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
    pub async fn add_reply(
        pool: &PgPool,
        review_id: &Uuid,
        user_id: &Uuid,
        reply: &str,
    ) -> Result<ReviewReply, AppError> {
        let reply_record = sqlx::query_as!(
            ReviewReply,
            r#"
            INSERT INTO review_replies (review_id, user_id, reply)
            VALUES ($1, $2, $3)
            RETURNING id, review_id, user_id, reply, created_at, updated_at
            "#,
            review_id,
            user_id,
            reply
        )
        .fetch_one(pool)
        .await?;
        
        Ok(reply_record)
    }
    
    pub async fn get_review_replies(
        pool: &PgPool,
        review_id: &Uuid,
    ) -> Result<Vec<ReviewReply>, AppError> {
        let replies = sqlx::query_as!(
            ReviewReply,
            r#"
            SELECT rr.id, rr.review_id, rr.user_id, rr.reply, rr.created_at, rr.updated_at
            FROM review_replies rr
            WHERE rr.review_id = $1
            ORDER BY rr.created_at ASC
            "#,
            review_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(replies)
    }
    
    pub async fn get_product_rating_summary(
        pool: &PgPool,
        product_id: &Uuid,
    ) -> Result<crate::dtos::review_dto::ProductRatingSummary, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT 
                COALESCE(AVG(rating), 0) as average_rating,
                COUNT(*) as total_reviews,
                COUNT(*) FILTER (WHERE rating = 5) as rating_5_stars,
                COUNT(*) FILTER (WHERE rating = 4) as rating_4_stars,
                COUNT(*) FILTER (WHERE rating = 3) as rating_3_stars,
                COUNT(*) FILTER (WHERE rating = 2) as rating_2_stars,
                COUNT(*) FILTER (WHERE rating = 1) as rating_1_stars
            FROM reviews
            WHERE product_id = $1 AND is_approved = true AND deleted_at IS NULL
            "#,
            product_id
        )
        .fetch_one(pool)
        .await?;
        
        let avg = result.average_rating.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0);
        
        Ok(crate::dtos::review_dto::ProductRatingSummary {
            average_rating: avg,
            total_reviews: result.total_reviews.unwrap_or(0),
            rating_5_stars: result.rating_5_stars.unwrap_or(0),
            rating_4_stars: result.rating_4_stars.unwrap_or(0),
            rating_3_stars: result.rating_3_stars.unwrap_or(0),
            rating_2_stars: result.rating_2_stars.unwrap_or(0),
            rating_1_stars: result.rating_1_stars.unwrap_or(0),
        })
    }
}
