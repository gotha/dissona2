/**
 * Samples Handler - Sample content for new users
 * 
 * Endpoints:
 * - POST /api/samples/try - Create a copy of sample content for user
 */

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;

#[derive(Debug, Serialize)]
pub struct SampleProject {
    pub id: Uuid,
    pub title: String,
    pub is_sample: bool,
    pub audiobook_status: String,
    pub chapters_count: i32,
}

#[derive(Debug, Serialize)]
pub struct TrySampleResponse {
    pub project: SampleProject,
}

/// Create a copy of sample content for the authenticated user
pub async fn try_sample(
    db: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, ApiError> {
    // Check if user already has the sample
    let existing = sqlx::query!(
        r#"
        SELECT id FROM projects 
        WHERE user_id = $1 AND is_sample = true
        "#,
        user.id
    )
    .fetch_optional(db.get_ref())
    .await?;

    if let Some(existing_project) = existing {
        // Return existing sample
        let project = sqlx::query!(
            r#"
            SELECT id, title, is_sample, audiobook_status,
                   (SELECT COUNT(*) FROM chapters WHERE project_id = projects.id)::int as chapters_count
            FROM projects
            WHERE id = $1
            "#,
            existing_project.id
        )
        .fetch_one(db.get_ref())
        .await?;

        return Ok(HttpResponse::Ok().json(TrySampleResponse {
            project: SampleProject {
                id: project.id,
                title: project.title,
                is_sample: project.is_sample,
                audiobook_status: project.audiobook_status.unwrap_or_default(),
                chapters_count: project.chapters_count.unwrap_or(0),
            },
        }));
    }

    // Create new sample project for user
    let project_id = Uuid::new_v4();
    let now = Utc::now();

    // Create the sample project
    sqlx::query!(
        r#"
        INSERT INTO projects (id, user_id, title, is_sample, audiobook_status, created_at, updated_at)
        VALUES ($1, $2, $3, true, 'ready', $4, $4)
        "#,
        project_id,
        user.id,
        "The Power of Focus (Sample)",
        now
    )
    .execute(db.get_ref())
    .await?;

    // Create sample chapters
    let sample_chapters = vec![
        ("Introduction", "An overview of why focus matters in the modern world."),
        ("The Science of Attention", "How our brains process information and maintain focus."),
        ("Digital Distractions", "Understanding and managing the constant pull of technology."),
        ("Building Focus Habits", "Practical techniques for developing better concentration."),
        ("Deep Work Strategies", "Methods for achieving flow states and meaningful productivity."),
    ];

    for (i, (title, summary)) in sample_chapters.iter().enumerate() {
        let chapter_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO chapters (id, project_id, title, summary, chapter_order, audio_status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'ready', $6, $6)
            "#,
            chapter_id,
            project_id,
            *title,
            *summary,
            (i + 1) as i32,
            now
        )
        .execute(db.get_ref())
        .await?;
    }

    Ok(HttpResponse::Created().json(TrySampleResponse {
        project: SampleProject {
            id: project_id,
            title: "The Power of Focus (Sample)".to_string(),
            is_sample: true,
            audiobook_status: "ready".to_string(),
            chapters_count: sample_chapters.len() as i32,
        },
    }))
}
