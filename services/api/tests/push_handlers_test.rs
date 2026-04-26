//! Integration tests for push notification handlers

mod helpers;

use actix_web::{http::StatusCode, test};
use uuid::Uuid;

use helpers::{cleanup_test_user, configure_test_app, create_test_token, get_test_db_pool};

#[actix_web::test]
async fn test_subscribe_requires_auth() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/push/subscribe")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "subscription": {
                "endpoint": "https://fcm.googleapis.com/fcm/send/test",
                "keys": {
                    "p256dh": "test-p256dh-key",
                    "auth": "test-auth-key"
                }
            }
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_subscribe_creates_subscription() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/push/subscribe")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "subscription": {
                "endpoint": "https://fcm.googleapis.com/fcm/send/test-123",
                "keys": {
                    "p256dh": "test-p256dh-key",
                    "auth": "test-auth-key"
                }
            }
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_subscribe_upserts_existing() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let endpoint = "https://fcm.googleapis.com/fcm/send/upsert-test";

    // First subscribe
    let req = test::TestRequest::post()
        .uri("/api/push/subscribe")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "subscription": {
                "endpoint": endpoint,
                "keys": { "p256dh": "key-v1", "auth": "auth-v1" }
            }
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Second subscribe with same endpoint but different keys
    let req = test::TestRequest::post()
        .uri("/api/push/subscribe")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "subscription": {
                "endpoint": endpoint,
                "keys": { "p256dh": "key-v2", "auth": "auth-v2" }
            }
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Should only have 1 subscription for this user/endpoint
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM push_subscriptions WHERE user_id = $1 AND endpoint = $2",
        user_id,
        endpoint
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count.unwrap_or(0), 1);

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_unsubscribe_removes_subscriptions() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    // Subscribe first
    let req = test::TestRequest::post()
        .uri("/api/push/subscribe")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "subscription": {
                "endpoint": "https://fcm.googleapis.com/fcm/send/unsub-test",
                "keys": { "p256dh": "key", "auth": "auth" }
            }
        }).to_string())
        .to_request();
    test::call_service(&app, req).await;

    // Unsubscribe
    let req = test::TestRequest::delete()
        .uri("/api/push/unsubscribe")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify no subscriptions remain
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM push_subscriptions WHERE user_id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count.unwrap_or(0), 0);

    cleanup_test_user(&pool, user_id).await;
}
