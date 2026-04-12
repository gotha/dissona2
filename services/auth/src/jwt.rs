use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AuthError;

#[derive(Clone)]
pub struct JwtConfig {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn create_access_token(&self, user_id: Uuid, email: &str, name: &str) -> Result<String, AuthError> {
        let expiration = Utc::now() + Duration::hours(1);
        
        let claims = AccessTokenClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            name: name.to_string(),
            iat: Utc::now().timestamp() as usize,
            exp: expiration.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn create_refresh_token(&self, user_id: Uuid, session_id: Uuid) -> Result<String, AuthError> {
        let expiration = Utc::now() + Duration::days(30);
        
        let claims = RefreshTokenClaims {
            sub: user_id.to_string(),
            session_id: session_id.to_string(),
            iat: Utc::now().timestamp() as usize,
            exp: expiration.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn verify_access_token(&self, token: &str) -> Result<AccessTokenClaims, AuthError> {
        decode::<AccessTokenClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, AuthError> {
        decode::<RefreshTokenClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,      // User ID
    pub email: String,
    pub name: String,
    pub iat: usize,       // Issued at
    pub exp: usize,       // Expiration
    pub jti: String,      // Token ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,          // User ID
    pub session_id: String,   // Session ID for revocation
    pub iat: usize,
    pub exp: usize,
    pub jti: String,
}
