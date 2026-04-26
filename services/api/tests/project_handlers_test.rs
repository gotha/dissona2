mod helpers;

use actix_web::test;
use actix_web::http::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use helpers::{cleanup_test_user, configure_test_app, create_test_project, create_test_chapter, get_test_db_pool, create_test_token};

// ── List Projects ──────────────────────────────────────────

#[actix_web::test]
async fn list_projects_requires_auth() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::get()
        .uri("/api/projects")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn list_projects_returns_empty_for_new_user() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri("/api/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn list_projects_returns_user_projects() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let _project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri("/api/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["title"], "Test Project");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn list_projects_does_not_return_other_users_projects() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let _project = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    let req = test::TestRequest::get()
        .uri("/api/projects")
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());

    cleanup_test_user(&pool, user_a).await;
    cleanup_test_user(&pool, user_b).await;
}

// ── Get Project ────────────────────────────────────────────

#[actix_web::test]
async fn get_project_returns_project() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "Test Project");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn get_project_returns_404_for_nonexistent() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}", Uuid::new_v4()))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_project_returns_404_for_other_user() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    cleanup_test_user(&pool, user_a).await;
}


// ── Update Project ─────────────────────────────────────────

#[actix_web::test]
async fn update_project_changes_title() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::put()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_json(serde_json::json!({"title": "New Title"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "New Title");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn update_project_returns_404_for_other_user() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    let req = test::TestRequest::put()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .insert_header(("Content-Type", "application/json"))
        .set_json(serde_json::json!({"title": "Hacked"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    cleanup_test_user(&pool, user_a).await;
}

// ── Delete Project ─────────────────────────────────────────

#[actix_web::test]
async fn delete_project_removes_project() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::delete()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status(), StatusCode::NOT_FOUND);

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn delete_project_returns_404_for_other_user() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    let req = test::TestRequest::delete()
        .uri(&format!("/api/projects/{}", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    cleanup_test_user(&pool, user_a).await;
}

// ── List Chapters ──────────────────────────────────────────

#[actix_web::test]
async fn list_chapters_returns_empty_for_new_project() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/chapters", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn list_chapters_returns_chapters() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_id).await;
    create_test_chapter(&pool, project_id).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/chapters", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["title"], "Test Chapter");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn list_chapters_returns_404_for_other_user() {
    let pool = get_test_db_pool().await;
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let project_id = create_test_project(&pool, user_a).await;
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token_b = create_test_token(user_b);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/chapters", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    cleanup_test_user(&pool, user_a).await;
}

// ── Create Project ─────────────────────────────────────────

#[actix_web::test]
async fn create_project_with_json() {
    let pool = get_test_db_pool().await;
    let user_id = Uuid::new_v4();
    let app = test::init_service(configure_test_app(pool.clone())).await;
    let token = create_test_token(user_id);

    let req = test::TestRequest::post()
        .uri("/api/projects")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_json(serde_json::json!({"title": "My Book", "description": "A description"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "My Book");
    assert_eq!(body["status"], "draft");

    cleanup_test_user(&pool, user_id).await;
}

#[actix_web::test]
async fn create_project_requires_auth() {
    let pool = get_test_db_pool().await;
    let app = test::init_service(configure_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/projects")
        .insert_header(("Content-Type", "application/json"))
        .set_json(serde_json::json!({"title": "No Auth"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}