use crate::models::product::Product;
use crate::config::redis::RedisClient;
use anyhow::Result;
use redis::AsyncCommands;

pub const PRODUCTS_CACHE_KEY: &str = "products:all";
pub const CACHE_TTL_SECONDS: u64 = 300;

pub async fn cache_all_products(
    redis_client: &RedisClient,
    products: &[Product],
) -> Result<()> {
    let mut conn = redis_client.connection_manager.clone();
    let serialized = serde_json::to_string(products)?;
    
    conn.set_ex::<_, _, ()>(PRODUCTS_CACHE_KEY, serialized, CACHE_TTL_SECONDS).await?;
    
    Ok(())
}

pub async fn get_cached_products(
    redis_client: &RedisClient,
) -> Result<Option<Vec<Product>>> {
    let mut conn = redis_client.connection_manager.clone();
    let result: Option<String> = conn.get(PRODUCTS_CACHE_KEY).await?;
    
    match result {
        Some(data) => {
            let products = serde_json::from_str(&data)?;
            Ok(Some(products))
        }
        None => Ok(None),
    }
}

pub async fn invalidate_products_cache(redis_client: &RedisClient) -> Result<()> {
    let mut conn = redis_client.connection_manager.clone();
    conn.del::<_, ()>(PRODUCTS_CACHE_KEY).await?;
    Ok(())
}