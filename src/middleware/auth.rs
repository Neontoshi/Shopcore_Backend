use axum::{
    extract::{State, Request},
    middleware::Next,
    response::{Response, IntoResponse},
    http::HeaderMap,
};
use crate::app::state::AppState;
use crate::utils::JwtService;
use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub role: String,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    let token = match extract_token_from_headers(headers) {
        Ok(t) => t,
        Err(e) => return e.into_response(),
    };

    let jwt_service = JwtService::new(
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    );

    let claims = match jwt_service.verify_access_token(&token) {
        Ok(c) => c,
        Err(_) => return AppError::unauthorized("Invalid token").into_response(),
    };

    let user_id = match uuid::Uuid::parse_str(&claims.claims.sub) {
        Ok(id) => id,
        Err(_) => return AppError::unauthorized("Invalid user ID").into_response(),
    };

    let auth_user = AuthUser {
        user_id,
        email: claims.claims.email,
        role: claims.claims.role,
    };

    request.extensions_mut().insert(auth_user);
    next.run(request).await
}

pub fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AppError::unauthorized("Missing authorization header"))?
        .to_str()
        .map_err(|_| AppError::unauthorized("Invalid authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized("Invalid token format"));
    }

    let token = auth_header[7..].to_string();
    Ok(token)
}