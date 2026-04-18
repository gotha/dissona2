//! JWT authentication for the API service
//!
//! Validates JWT tokens using the shared JWT_SECRET.
//! Does not require any connection to the Auth Service.

use actix_web::{web, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::error::ApiError;

/// JWT claims structure (must match Auth Service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (UUID as string)
    pub sub: String,
    /// User email
    pub email: String,
    /// User name
    pub name: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration (Unix timestamp)
    pub exp: i64,
    /// Token ID
    pub jti: String,
}

/// Authenticated user extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

impl TryFrom<Claims> for AuthenticatedUser {
    type Error = ApiError;

    fn try_from(claims: Claims) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&claims.sub)
            .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".into()))?;

        Ok(Self {
            id,
            email: claims.email,
            name: claims.name,
        })
    }
}

/// JWT validator
#[derive(Clone)]
pub struct JwtValidator {
    decoding_key: DecodingKey,
}

impl JwtValidator {
    pub fn new(secret: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    /// Validate a JWT token and return the claims
    pub fn validate(&self, token: &str) -> Result<Claims, ApiError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| ApiError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}

/// Extract the JWT token from the Authorization header
pub fn extract_token(req: &HttpRequest) -> Result<String, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?
        .to_str()
        .map_err(|_| ApiError::Unauthorized("Invalid Authorization header".into()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::Unauthorized("Invalid Authorization header format".into()))?;

    Ok(token.to_string())
}

/// Extract authenticated user from request
pub fn get_authenticated_user(
    req: &HttpRequest,
    validator: &JwtValidator,
) -> Result<AuthenticatedUser, ApiError> {
    let token = extract_token(req)?;
    let claims = validator.validate(&token)?;
    AuthenticatedUser::try_from(claims)
}

/// Implement FromRequest for AuthenticatedUser to enable extraction in handlers
impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt = req
            .app_data::<web::Data<JwtValidator>>()
            .expect("JwtValidator not configured");

        match get_authenticated_user(req, jwt.get_ref()) {
            Ok(user) => ready(Ok(user)),
            Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e.to_string()))),
        }
    }
}
