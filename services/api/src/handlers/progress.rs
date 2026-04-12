/**
 * Progress Handler - Cross-device playback position sync
 * 
 * Endpoints:
 * - GET /api/projects/{id}/progress - Get current progress
 * - PUT /api/projects/{id}/progress - Update progress
 */

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;

#[derive(Debug, Serialize)]
pub struct ProgressResponse {
    pub project_id: Uuid,
    pub chapter_id: Option<Uuid>,
    pub position_ms: i64,
    pub listening_mode: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub chapter_id: Uuid,
    pub position_ms: i64,
    pub listening_mode: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateProgressResponse {
    pub updated_at: DateTime<Utc>,
}

/// Get playback progress for a project
pub async fn get_progress(
    path: web::Path<Uuid>,
    db: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, ApiError> {
    let project_id = path.into_inner();

    // Verify user owns the project or has access
    let project = sqlx::query!(
        r#"
        SELECT id FROM projects WHERE id = $1 AND user_id = $2
        "#,
        project_id,
        user.id
    )
    .fetch_optional(db.get_ref())
    .await?;

    if project.is_none() {
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // Get progress
    let progress = sqlx::query!(
        r#"
        SELECT current_chapter_id, position_ms, listening_mode, updated_at
        FROM playback_progress
        WHERE project_id = $1 AND user_id = $2
        "#,
        project_id,
        user.id
    )
    .fetch_optional(db.get_ref())
    .await?;

    match progress {
        Some(p) => Ok(HttpResponse::Ok().json(ProgressResponse {
            project_id,
            chapter_id: p.current_chapter_id,
            position_ms: p.position_ms.unwrap_or(0) as i64,
            listening_mode: p.listening_mode.unwrap_or_else(|| "blitz".to_string()),
            updated_at: p.updated_at,
        })),
        None => Ok(HttpResponse::Ok().json(ProgressResponse {
            project_id,
            chapter_id: None,
            position_ms: 0,
            listening_mode: "blitz".to_string(),
            updated_at: Utc::now(),
        })),
    }
}

/// Update playback progress for a project
pub async fn update_progress(
    path: web::Path<Uuid>,
    body: web::Json<UpdateProgressRequest>,
    db: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, ApiError> {
    let project_id = path.into_inner();

    // Verify user owns the project or has access
    let project = sqlx::query!(
        r#"
        SELECT id FROM projects WHERE id = $1 AND user_id = $2
        "#,
        project_id,
        user.id
    )
    .fetch_optional(db.get_ref())
    .await?;

    if project.is_none() {
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // Upsert progress
    let updated_at = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO playback_progress (project_id, user_id, current_chapter_id, position_ms, listening_mode, updated_at, last_played_at)
        VALUES ($1, $2, $3, $4, $5, $6, $6)
        ON CONFLICT (project_id, user_id)
        DO UPDATE SET
            current_chapter_id = EXCLUDED.current_chapter_id,
            position_ms = EXCLUDED.position_ms,
            listening_mode = EXCLUDED.listening_mode,
            updated_at = EXCLUDED.updated_at,
            last_played_at = EXCLUDED.last_played_at
        "#,
        project_id,
        user.id,
        body.chapter_id,
        body.position_ms as i32,
        body.listening_mode,
        updated_at
    )
    .execute(db.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(UpdateProgressResponse { updated_at }))
}
