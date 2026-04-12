/**
 * Push Notification Handler
 * 
 * Endpoints:
 * - POST /api/push/subscribe - Subscribe to push notifications
 * - DELETE /api/push/unsubscribe - Unsubscribe from push notifications
 */

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct PushKeys {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Debug, Deserialize)]
pub struct PushSubscriptionPayload {
    pub endpoint: String,
    pub keys: PushKeys,
}

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub subscription: PushSubscriptionPayload,
}

#[derive(Debug, Serialize)]
pub struct SubscribeResponse {
    pub id: Uuid,
    pub created_at: String,
}

/// Subscribe to push notifications
pub async fn subscribe(
    body: web::Json<SubscribeRequest>,
    db: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, ApiError> {
    let subscription = &body.subscription;
    let now = Utc::now();
    let id = Uuid::new_v4();

    // Upsert subscription (replace if same endpoint exists)
    sqlx::query!(
        r#"
        INSERT INTO push_subscriptions (id, user_id, endpoint, p256dh, auth, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, endpoint)
        DO UPDATE SET
            p256dh = EXCLUDED.p256dh,
            auth = EXCLUDED.auth,
            created_at = EXCLUDED.created_at
        RETURNING id
        "#,
        id,
        user.id,
        subscription.endpoint,
        subscription.keys.p256dh,
        subscription.keys.auth,
        now
    )
    .fetch_one(db.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(SubscribeResponse {
        id,
        created_at: now.to_rfc3339(),
    }))
}

/// Unsubscribe from push notifications
pub async fn unsubscribe(
    db: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<impl Responder, ApiError> {
    // Delete all subscriptions for this user
    sqlx::query!(
        r#"
        DELETE FROM push_subscriptions WHERE user_id = $1
        "#,
        user.id
    )
    .execute(db.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Unsubscribed"
    })))
}
