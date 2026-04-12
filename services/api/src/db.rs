// Database utilities and helpers
// This module will contain shared database operations
//
// Note: user_id is a UUID that references users in the Auth Service database.
// We don't have a local users table - user info comes from JWT tokens.

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ApiError;

/// Check if a user owns a project
pub async fn verify_project_ownership(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ApiError> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1 AND user_id = $2)",
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Check if a user has access to a project (owner or shared)
pub async fn verify_project_access(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ApiError> {
    // First check ownership
    if verify_project_ownership(pool, project_id, user_id).await? {
        return Ok(true);
    }

    // Then check shares
    let shared = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM shares s
            JOIN share_redemptions sr ON sr.share_id = s.id
            WHERE s.project_id = $1 AND sr.user_id = $2 AND s.is_active = true
        )
        "#,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(shared.unwrap_or(false))
}
