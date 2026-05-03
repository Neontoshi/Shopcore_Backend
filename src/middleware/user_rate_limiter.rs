use tower_governor::{
    key_extractor::KeyExtractor,
    GovernorError,
};
use axum::http::Request;
use crate::middleware::auth::AuthUser;

#[derive(Clone)]
pub struct UserIdKeyExtractor;

impl KeyExtractor for UserIdKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        req.extensions()
            .get::<AuthUser>()
            .map(|user| user.user_id.to_string())
            .ok_or(GovernorError::UnableToExtractKey)
    }
}