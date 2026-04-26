use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bytes::BytesMut;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::{get_authenticated_user, JwtValidator};
use crate::error::ApiError;
use crate::nats::{self, PdfParseJob};
use crate::s3::StorageClient;

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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ChapterResponse {
    pub id: Uuid,
    pub title: Option<String>,
    pub chapter_number: Option<i32>,
    pub word_count: Option<i32>,
    pub status: String,
}

pub async fn list_chapters(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    let project_id = path.into_inner();

    // Verify ownership
    let _project = sqlx::query!("SELECT id FROM projects WHERE id = $1 AND user_id = $2", project_id, user.id)
        .fetch_optional(db.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("Project not found".to_string()))?;

    let chapters = sqlx::query_as!(
        ChapterResponse,
        r#"SELECT id, title, chapter_number, word_count, status
           FROM chapters WHERE project_id = $1 ORDER BY chapter_number ASC"#,
        project_id
    )
    .fetch_all(db.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(chapters))
}


const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB

/// Upload a PDF and create a project with document
pub async fn upload_project(
    req: HttpRequest,
    db: web::Data<PgPool>,
    jwt: web::Data<JwtValidator>,
    storage: web::Data<StorageClient>,
    nats_js: web::Data<async_nats::jetstream::Context>,
    mut payload: actix_web::web::Payload,
) -> Result<impl Responder, ApiError> {
    let user = get_authenticated_user(&req, &jwt)?;
    let user_id = user.id;

    // Read content-type and get filename from header
    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !content_type.starts_with("application/pdf") {
        return Err(ApiError::BadRequest("Only PDF files are supported".into()));
    }

    let filename = req
        .headers()
        .get("x-filename")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("document.pdf")
        .to_string();

    // Read body bytes with size limit
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| ApiError::BadRequest(format!("Read error: {}", e)))?;
        if body.len() + chunk.len() > MAX_FILE_SIZE {
            return Err(ApiError::BadRequest(
                "File too large (max 100MB)".into(),
            ));
        }
        body.extend_from_slice(&chunk);
    }

    let file_data = body.freeze();
    let file_size = file_data.len() as i64;

    // Create project
    let title = filename.trim_end_matches(".pdf").to_string();
    let project_id = Uuid::new_v4();
    let document_id = Uuid::new_v4();

    sqlx::query!(
        r#"INSERT INTO projects (id, user_id, title, status) VALUES ($1, $2, $3, 'processing')"#,
        project_id,
        user_id,
        title
    )
    .execute(db.get_ref())
    .await?;

    // Upload to S3
    let s3_key = format!("{}/original.pdf", project_id);
    storage
        .upload_file(&s3_key, file_data, "application/pdf")
        .await
        .map_err(|e| ApiError::Internal(format!("S3 upload failed: {}", e)))?;

    // Create document record
    let file_size_i32 = file_size as i32;
    sqlx::query!(
        r#"INSERT INTO documents (id, project_id, position, title, source_type, source_file_path, source_file_size, status)
           VALUES ($1, $2, 1, $3, 'pdf', $4, $5, 'uploaded')"#,
        document_id,
        project_id,
        title,
        s3_key,
        file_size_i32
    )
    .execute(db.get_ref())
    .await?;

    // Publish NATS job
    let job = PdfParseJob {
        job_id: Uuid::new_v4(),
        document_id,
        project_id,
        file_path: s3_key,
    };
    nats::publish_job(nats_js.get_ref(), nats::subjects::PDF_PARSE, &job)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to publish job: {}", e)))?;

    // Return project
    let project = sqlx::query_as!(
        ProjectResponse,
        r#"SELECT id, title, description, status, audiobook_status, podcast_status, created_at
           FROM projects WHERE id = $1"#,
        project_id
    )
    .fetch_one(db.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(project))
}

pub async fn add_document() -> Result<impl Responder, ApiError> {
    // TODO: Implement adding additional documents to existing project
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
