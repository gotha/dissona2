//! Tests for authentication handlers and JWT functionality
//!
//! Unit tests verify JWT creation, validation, and claims.
//! Integration tests (marked with #[ignore]) require a running database.

use dissona_auth::jwt::JwtConfig;
use uuid::Uuid;

#[cfg(test)]
mod jwt_tests {
    use super::*;

    fn test_jwt_config() -> JwtConfig {
        JwtConfig::new("test-secret-key-for-unit-tests-only")
    }

    #[test]
    fn test_access_token_contains_correct_claims() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();

        let token = config
            .create_access_token(user_id, "test@example.com", "Test User", false)
            .expect("Failed to create access token");

        let claims = config
            .verify_access_token(&token)
            .expect("Failed to verify access token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.name, "Test User");
        assert!(!claims.has_completed_first_upload);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_access_token_has_completed_first_upload_flag() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();

        let token = config
            .create_access_token(user_id, "test@example.com", "Test User", true)
            .expect("Failed to create access token");

        let claims = config
            .verify_access_token(&token)
            .expect("Failed to verify access token");

        assert!(claims.has_completed_first_upload);
    }

    #[test]
    fn test_access_token_expires_in_one_hour() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();

        let token = config
            .create_access_token(user_id, "test@example.com", "Test", false)
            .expect("Failed to create access token");

        let claims = config
            .verify_access_token(&token)
            .expect("Failed to verify");

        let duration = claims.exp - claims.iat;
        // Should be approximately 3600 seconds (1 hour)
        assert!(duration >= 3590 && duration <= 3610);
    }

    #[test]
    fn test_refresh_token_contains_user_id() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let token = config
            .create_refresh_token(user_id, session_id)
            .expect("Failed to create refresh token");

        let claims = config
            .verify_refresh_token(&token)
            .expect("Failed to verify refresh token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.session_id, session_id.to_string());
    }

    #[test]
    fn test_refresh_token_expires_in_30_days() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let token = config
            .create_refresh_token(user_id, session_id)
            .expect("Failed to create refresh token");

        let claims = config
            .verify_refresh_token(&token)
            .expect("Failed to verify");

        let duration_days = (claims.exp - claims.iat) / 86400;
        assert_eq!(duration_days, 30);
    }

    #[test]
    fn test_access_token_invalid_with_wrong_secret() {
        let config1 = test_jwt_config();
        let config2 = JwtConfig::new("different-secret-key");
        let user_id = Uuid::new_v4();

        let token = config1
            .create_access_token(user_id, "test@example.com", "Test", false)
            .expect("Failed to create access token");

        let result = config2.verify_access_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token_invalid_with_wrong_secret() {
        let config1 = test_jwt_config();
        let config2 = JwtConfig::new("different-secret-key");
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let token = config1
            .create_refresh_token(user_id, session_id)
            .expect("Failed to create refresh token");

        let result = config2.verify_refresh_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_token_rejected() {
        let config = test_jwt_config();

        let result = config.verify_access_token("not-a-valid-jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_each_token_has_unique_jti() {
        let config = test_jwt_config();
        let user_id = Uuid::new_v4();

        let token1 = config
            .create_access_token(user_id, "test@example.com", "Test", false)
            .unwrap();
        let token2 = config
            .create_access_token(user_id, "test@example.com", "Test", false)
            .unwrap();

        let claims1 = config.verify_access_token(&token1).unwrap();
        let claims2 = config.verify_access_token(&token2).unwrap();

        assert_ne!(claims1.jti, claims2.jti);
    }
}
