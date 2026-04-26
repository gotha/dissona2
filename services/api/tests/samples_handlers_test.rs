//! Integration tests for sample content handlers

mod helpers;

use actix_web::{http::StatusCode, test};
use uuid::Uuid;

use helpers::{cleanup_test_user, configure_test_app, create_test_token, get_test_db_pool};

#[actix_web::test]
async fn test_try_sample_requires_auth() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/samples/try")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_try_sample_creates_project() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/samples/try")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["project"]["is_sample"], true);
    assert_eq!(body["project"]["chapters_count"], 5);
    assert_eq!(body["project"]["audiobook_status"], "ready");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_try_sample_returns_existing() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    // First call creates
    let req = test::TestRequest::post()
        .uri("/api/samples/try")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body1: serde_json::Value = test::read_body_json(resp).await;

    // Second call returns existing
    let req = test::TestRequest::post()
        .uri("/api/samples/try")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body2: serde_json::Value = test::read_body_json(resp).await;

    // Same project ID returned
    assert_eq!(body1["project"]["id"], body2["project"]["id"]);

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn test_sample_has_chapters() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let token = create_test_token(user_id);
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/samples/try")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["project"]["chapters_count"], 5);

    // Verify chapters exist in DB
    let project_id: Uuid = body["project"]["id"].as_str().unwrap().parse().unwrap();
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM chapters WHERE project_id = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count.unwrap_or(0), 5);

    cleanup_test_user(&pool, user_id).await;
}
