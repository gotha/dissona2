//! Shared test helpers for API integration tests

use actix_web::{web, App};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

use dissona_api::auth::{Claims, JwtValidator};
use dissona_api::handlers;

/// Test JWT secret - must match what we configure in tests
pub const TEST_JWT_SECRET: &str = "test-api-integration-secret";

/// Create a valid JWT token for testing
pub fn create_test_token(user_id: Uuid) -> String {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        iat: now,
        exp: now + 3600,
        jti: Uuid::new_v4().to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TEST_JWT_SECRET.as_bytes()),
    )
    .expect("Failed to create test JWT")
}

/// Get a database pool from DATABASE_URL env var
pub async fn get_test_db_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for integration tests");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

/// Configure a test actix-web app with real DB and test JWT
pub fn configure_test_app(
    db_pool: PgPool,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let jwt_validator = JwtValidator::new(TEST_JWT_SECRET);
    App::new()
        .app_data(web::Data::new(db_pool))
        .app_data(web::Data::new(jwt_validator))
        .configure(handlers::configure)
}

/// Clean up test data for a specific user
pub async fn cleanup_test_user(pool: &PgPool, user_id: Uuid) {
    let _ = sqlx::query("DELETE FROM playback_progress WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await;
    let _ = sqlx::query("DELETE FROM push_subscriptions WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await;
    // chapters and documents cascade from projects
    let _ = sqlx::query("DELETE FROM projects WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await;
}

/// Create a test project for a given user, returns the project ID
pub async fn create_test_project(pool: &PgPool, user_id: Uuid) -> Uuid {
    let project_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO projects (id, user_id, title, status) VALUES ($1, $2, 'Test Project', 'draft')"
    )
    .bind(project_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to create test project");
    project_id
}

/// Create a test document for a given project, returns the document ID
pub async fn create_test_document(pool: &PgPool, project_id: Uuid) -> Uuid {
    let doc_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO documents (id, project_id, position, title, source_type, status) VALUES ($1, $2, 1, 'Test Doc', 'text', 'ready')"
    )
    .bind(doc_id)
    .bind(project_id)
    .execute(pool)
    .await
    .expect("Failed to create test document");
    doc_id
}

/// Create a test chapter for a given project, returns the chapter ID
pub async fn create_test_chapter(pool: &PgPool, project_id: Uuid) -> Uuid {
    let doc_id = create_test_document(pool, project_id).await;
    let chapter_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO chapters (id, document_id, project_id, chapter_number, title, status) VALUES ($1, $2, $3, 1, 'Test Chapter', 'ready')"
    )
    .bind(chapter_id)
    .bind(doc_id)
    .bind(project_id)
    .execute(pool)
    .await
    .expect("Failed to create test chapter");
    chapter_id
}
