use actix_web::{cookie::{Cookie, SameSite}, web, HttpResponse, Responder};
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AuthError;
use crate::jwt::JwtConfig;
use crate::oauth::get_google_user_info;

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

pub async fn google_login(oauth_client: web::Data<BasicClient>) -> impl Responder {
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    HttpResponse::Found()
        .insert_header(("Location", auth_url.to_string()))
        .finish()
}

pub async fn google_callback(
    query: web::Query<CallbackQuery>,
    oauth_client: web::Data<BasicClient>,
    db: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
) -> Result<impl Responder, AuthError> {
    // Exchange code for token
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| AuthError::OAuthError(e.to_string()))?;

    let access_token = token_result.access_token().secret();

    // Get user info from Google
    let user_info = get_google_user_info(access_token)
        .await
        .map_err(|e| AuthError::OAuthError(e.to_string()))?;

    // Find or create user
    let user = sqlx::query!(
        r#"
        INSERT INTO users (email, name, google_id, avatar_url)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (google_id) DO UPDATE
        SET name = EXCLUDED.name, avatar_url = EXCLUDED.avatar_url, updated_at = NOW()
        RETURNING id, email, name
        "#,
        user_info.email,
        user_info.name,
        user_info.id,
        user_info.picture
    )
    .fetch_one(db.get_ref())
    .await?;

    // Create tokens
    let access_token = jwt_config.create_access_token(
        user.id,
        &user.email,
        user.name.as_deref().unwrap_or(""),
    )?;

    let session_id = Uuid::new_v4();
    let refresh_token = jwt_config.create_refresh_token(user.id, session_id)?;

    // Set refresh token as HttpOnly cookie
    let refresh_cookie = Cookie::build("refresh_token", refresh_token)
        .path("/auth")
        .http_only(true)
        .secure(true) // Set to false for local dev without HTTPS
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .finish();

    // Redirect to frontend with access token
    let redirect_url = format!(
        "{}/auth/callback?access_token={}",
        settings.frontend.url,
        access_token
    );

    Ok(HttpResponse::Found()
        .cookie(refresh_cookie)
        .insert_header(("Location", redirect_url))
        .finish())
}
