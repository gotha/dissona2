use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::{get_authenticated_user, JwtValidator};
use crate::error::ApiError;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub audiobook_status: Option<String>,
    pub podcast_status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_projects(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    let user_id = user.id;

    let projects = sqlx::query_as!(
        ProjectResponse,
        r#"
        SELECT id, title, description, status, audiobook_status, podcast_status, created_at
        FROM projects
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(db.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(projects))
}

pub async fn create_project(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    body: web::Json<CreateProjectRequest>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let user_id = user.id;

    let project = sqlx::query_as!(
        ProjectResponse,
        r#"
        INSERT INTO projects (user_id, title, description)
        VALUES ($1, $2, $3)
        RETURNING id, title, description, status, audiobook_status, podcast_status, created_at
        "#,
        user_id,
        body.title,
        body.description
    )
    .fetch_one(db.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(project))
}

pub async fn get_project(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    let project_id = path.into_inner();
    let user_id = user.id;

    let project = sqlx::query_as!(
        ProjectResponse,
        r#"
        SELECT id, title, description, status, audiobook_status, podcast_status, created_at
        FROM projects
        WHERE id = $1 AND user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_optional(db.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Project not found".to_string()))?;

    Ok(HttpResponse::Ok().json(project))
}

pub async fn update_project(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    path: web::Path<Uuid>,
    body: web::Json<CreateProjectRequest>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let project_id = path.into_inner();
    let user_id = user.id;

    let project = sqlx::query_as!(
        ProjectResponse,
        r#"
        UPDATE projects
        SET title = $1, description = $2, updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING id, title, description, status, audiobook_status, podcast_status, created_at
        "#,
        body.title,
        body.description,
        project_id,
        user_id
    )
    .fetch_optional(db.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Project not found".to_string()))?;

    Ok(HttpResponse::Ok().json(project))
}

pub async fn delete_project(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    let project_id = path.into_inner();
    let user_id = user.id;

    let result = sqlx::query!(
        "DELETE FROM projects WHERE id = $1 AND user_id = $2",
        project_id,
        user_id
    )
    .execute(db.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Project not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn add_document() -> Result<impl Responder, ApiError> {
    // TODO: Implement document upload
    Ok(HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Not implemented yet"
    })))
}

pub async fn generate_audiobook() -> Result<impl Responder, ApiError> {
    // TODO: Implement audiobook generation
    Ok(HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Not implemented yet"
    })))
}

pub async fn generate_podcast() -> Result<impl Responder, ApiError> {
    // TODO: Implement podcast generation
    Ok(HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Not implemented yet"
    })))
}
