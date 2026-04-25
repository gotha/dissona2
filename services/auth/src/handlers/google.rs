use actix_web::{cookie::{Cookie, SameSite}, web, HttpRequest, HttpResponse, Responder};
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Settings;
use crate::error::AuthError;
use crate::jwt::JwtConfig;
use crate::oauth::get_google_user_info;

/// Cookie name for CSRF state parameter
const CSRF_COOKIE_NAME: &str = "oauth_state";

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

pub async fn google_login(oauth_client: web::Data<BasicClient>) -> impl Responder {
    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // Store CSRF token in an HttpOnly cookie so we can validate on callback
    let state_cookie = Cookie::build(CSRF_COOKIE_NAME, csrf_token.secret().clone())
        .path("/auth")
        .http_only(true)
        .same_site(SameSite::Lax) // Lax required for cross-site redirect
        .max_age(actix_web::cookie::time::Duration::minutes(10))
        .finish();

    HttpResponse::Found()
        .cookie(state_cookie)
        .insert_header(("Location", auth_url.to_string()))
        .finish()
}

pub async fn google_callback(
    req: HttpRequest,
    query: web::Query<CallbackQuery>,
    oauth_client: web::Data<BasicClient>,
    db: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    settings: web::Data<Settings>,
) -> Result<impl Responder, AuthError> {
    // Validate CSRF state parameter against the stored cookie
    let stored_state = req
        .cookie(CSRF_COOKIE_NAME)
        .ok_or_else(|| AuthError::OAuthError("Missing OAuth state cookie".to_string()))?;

    if query.state != stored_state.value() {
        return Err(AuthError::OAuthError("Invalid OAuth state parameter".to_string()));
    }

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
        RETURNING id, email, name, has_completed_first_upload
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
        user.has_completed_first_upload,
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

    // Clear the CSRF state cookie
    let mut clear_state_cookie = Cookie::build(CSRF_COOKIE_NAME, "")
        .path("/auth")
        .http_only(true)
        .finish();
    clear_state_cookie.make_removal();

    Ok(HttpResponse::Found()
        .cookie(refresh_cookie)
        .cookie(clear_state_cookie)
        .insert_header(("Location", redirect_url))
        .finish())
}
