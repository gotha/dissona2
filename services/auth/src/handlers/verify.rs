use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::error::AuthError;
use crate::jwt::JwtConfig;

pub async fn verify_token(
    req: HttpRequest,
    jwt_config: web::Data<JwtConfig>,
) -> Result<impl Responder, AuthError> {
    // Get token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AuthError::InvalidToken("No authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AuthError::InvalidToken("Invalid authorization header".to_string()))?;

    // Verify token
    let claims = jwt_config.verify_access_token(token)?;

    // Return user info (for forward auth in Traefik)
    Ok(HttpResponse::Ok()
        .insert_header(("X-User-Id", claims.sub.clone()))
        .insert_header(("X-User-Email", claims.email.clone()))
        .insert_header(("X-User-Name", claims.name.clone()))
        .json(serde_json::json!({
            "user_id": claims.sub,
            "email": claims.email,
            "name": claims.name,
            "hasCompletedFirstUpload": claims.has_completed_first_upload
        })))
}
