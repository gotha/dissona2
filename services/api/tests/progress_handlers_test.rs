//! Integration tests for progress sync handlers

mod helpers;

use actix_web::{http::StatusCode, test};
use uuid::Uuid;

use helpers::{cleanup_test_user, configure_test_app, create_test_chapter, create_test_project, create_test_token, get_test_db_pool};

#[actix_web::test]
async fn test_get_progress_requires_auth() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let project_id = Uuid::new_v4();

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/progress", project_id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_get_progress_returns_404_for_nonexistent_project() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/progress", Uuid::new_v4()))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_get_progress_returns_ok_for_new_project() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/progress", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_update_progress_saves_and_retrieves() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let chapter_id = create_test_chapter(&pool, project_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    // Save progress
    let req = test::TestRequest::put()
        .uri(&format!("/api/projects/{}/progress", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "chapter_id": chapter_id,
            "position_ms": 45000,
            "listening_mode": "audiobook"
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Retrieve progress
    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/progress", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["position_ms"], 45000);
    assert_eq!(body["listening_mode"], "audiobook");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_update_progress_invalid_project_returns_error() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);

    let req = test::TestRequest::put()
        .uri(&format!("/api/projects/{}/progress", Uuid::new_v4()))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(serde_json::json!({
            "chapter_id": Uuid::new_v4(),
            "position_ms": 0,
            "listening_mode": "audiobook"
        }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_progress_is_user_scoped() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    // User B tries to access User A's project progress
    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/progress", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Should be 404 (not found for this user) or 403
    assert!(
        resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::FORBIDDEN,
        "Expected 404 or 403, got {}",
        resp.status()
    );

    cleanup_test_user(&pool, user_a).await;
    cleanup_test_user(&pool, user_b).await;
}
