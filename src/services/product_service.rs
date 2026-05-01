use crate::models::Product;
use crate::repositories::ProductRepository;
use crate::dtos::{CreateProductRequest, UpdateProductRequest, ProductResponse, CategoryInfo};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct ProductService;

impl ProductService {
    pub async fn create_product(
        pool: &PgPool,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        // Check if product with same slug exists
        if let Some(existing) = ProductRepository::find_by_slug(pool, &req.slug).await? {
            if existing.slug == req.slug {
                return Err(AppError::conflict("Product with this slug already exists"));
            }
        }
        
        let product = Product {
            id: Uuid::new_v4(),
            name: req.name,
            slug: req.slug,
            description: req.description,
            price: req.price,
            compare_at_price: req.compare_at_price,
            stock_quantity: req.stock_quantity,
            category_id: req.category_id,
            sku: req.sku,
            is_active: true,
            image_url: req.image_url,
            average_rating: Decimal::ZERO,
            total_reviews: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        ProductRepository::create(pool, &product).await?;
        
        Ok(product.into())
    }
    
    pub async fn get_product(pool: &PgPool, id: &Uuid) -> Result<ProductResponse, AppError> {
        let product = ProductRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::not_found("Product"))?;
        
        // Fetch category info if category_id exists
        let category = if let Some(cat_id) = product.category_id {
            crate::repositories::CategoryRepository::find_by_id(pool, &cat_id)
                .await?
                .map(|c| CategoryInfo {
                    id: c.id,
                    name: c.name,
                    slug: c.slug,
                })
        } else {
            None
        };
        
        Ok(ProductResponse {
            weight: None,
            id: product.id,
            name: product.name,
            slug: product.slug,
            description: product.description,
            price: product.price,
            compare_at_price: product.compare_at_price,
            stock_quantity: product.stock_quantity,
            category_id: product.category_id,
            sku: product.sku,
            is_active: product.is_active,
            image_url: product.image_url,
            average_rating: Some(product.average_rating),
            total_reviews: Some(product.total_reviews),
            created_at: product.created_at,
            updated_at: product.updated_at,
            category,
        })
    }
    
    pub async fn get_product_by_slug(pool: &PgPool, slug: &str) -> Result<ProductResponse, AppError> {
        let product = ProductRepository::find_by_slug(pool, slug)
            .await?
            .ok_or_else(|| AppError::not_found("Product"))?;
        
        Ok(product.into())
    }
    
    pub async fn update_product(
        pool: &PgPool,
        id: &Uuid,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let mut product = ProductRepository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::not_found("Product"))?;
        
        if let Some(name) = req.name {
            product.name = name;
        }
        
        if let Some(slug) = req.slug {
            product.slug = slug;
        }
        
        if let Some(description) = req.description {
            product.description = Some(description);
        }
        
        if let Some(price) = req.price {
            product.price = price;
        }
        
        if let Some(compare_at_price) = req.compare_at_price {
            product.compare_at_price = Some(compare_at_price);
        }
        
        if let Some(stock_quantity) = req.stock_quantity {
            product.stock_quantity = stock_quantity;
        }
        
        if let Some(category_id) = req.category_id {
            product.category_id = Some(category_id);
        }
        
        if let Some(is_active) = req.is_active {
            product.is_active = is_active;
        }
        
        if let Some(image_url) = req.image_url {
            product.image_url = Some(image_url);
        }
        
        product.updated_at = Utc::now();
        
        ProductRepository::update(pool, &product).await?;
        
        Ok(product.into())
    }
    
    pub async fn delete_product(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        ProductRepository::delete(pool, id).await?;
        Ok(())
    }
}

// Update the From<Product> implementation
impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        ProductResponse {
        weight: None,
            id: product.id,
            name: product.name,
            slug: product.slug,
            description: product.description,
            price: product.price,
            compare_at_price: product.compare_at_price,
            stock_quantity: product.stock_quantity,
            category_id: product.category_id,
            sku: product.sku,
            is_active: product.is_active,
            image_url: product.image_url,
            average_rating: Some(product.average_rating),
            total_reviews: Some(product.total_reviews),
            created_at: product.created_at,
            updated_at: product.updated_at,
            category: None,
        }
    }
}
