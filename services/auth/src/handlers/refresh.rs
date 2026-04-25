use actix_web::{cookie::{Cookie, SameSite}, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AuthError;
use crate::jwt::JwtConfig;

pub async fn refresh_token(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
) -> Result<impl Responder, AuthError> {
    // Get refresh token from cookie
    let refresh_token = req
        .cookie("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| AuthError::InvalidToken("No refresh token".to_string()))?;

    // Verify refresh token
    let claims = jwt_config.verify_refresh_token(&refresh_token)?;

    let user_id: Uuid = claims
        .sub
        .parse()
        .map_err(|_| AuthError::InvalidToken("Invalid user ID".to_string()))?;

    // Get user from database
    let user = sqlx::query!(
        "SELECT id, email, name, has_completed_first_upload FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(db.get_ref())
    .await?
    .ok_or_else(|| AuthError::InvalidToken("User not found".to_string()))?;

    // Create new access token
    let access_token = jwt_config.create_access_token(
        user.id,
        &user.email,
        user.name.as_deref().unwrap_or(""),
        user.has_completed_first_upload,
    )?;

    // Create new refresh token (rotation)
    let new_session_id = Uuid::new_v4();
    let new_refresh_token = jwt_config.create_refresh_token(user.id, new_session_id)?;

    // Set new refresh token cookie
    let refresh_cookie = Cookie::build("refresh_token", new_refresh_token)
        .path("/auth")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(serde_json::json!({
            "access_token": access_token
        })))
}

pub async fn logout(_req: HttpRequest) -> impl Responder {
    // Clear refresh token cookie
    let mut removal_cookie = Cookie::build("refresh_token", "")
        .path("/auth")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .finish();
    removal_cookie.make_removal();

    HttpResponse::Ok()
        .cookie(removal_cookie)
        .json(serde_json::json!({
            "message": "Logged out"
        }))
}
