// Domain models
// These represent the core entities in the system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub cover_image_path: Option<String>,
    pub voice_id: Option<Uuid>,
    pub status: String,
    pub substatus: Option<String>,
    pub documents_total: i32,
    pub documents_processed: i32,
    pub chapters_total: i32,
    pub chapters_ready: i32,
    pub audiobook_status: Option<String>,
    pub podcast_status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: Uuid,
    pub project_id: Uuid,
    pub position: i32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub source_file_path: Option<String>,
    pub source_file_size: Option<i32>,
    pub source_type: Option<String>,
    pub page_count: Option<i32>,
    pub status: String,
    pub substatus: Option<String>,
    pub detection_method: Option<String>,
    pub extracted_chapters_count: i32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Chapter {
    pub id: Uuid,
    pub document_id: Uuid,
    pub project_id: Uuid,
    pub chapter_number: i32,
    pub title: Option<String>,
    pub source_text: Option<String>,
    pub word_count: Option<i32>,
    pub estimated_duration_min: Option<i32>,
    pub status: String,
    pub substatus: Option<String>,
    pub segments_total: i32,
    pub key_points_total: i32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Segment {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub segment_number: i32,
    pub text: String,
    pub word_count: Option<i32>,
    pub source_start_char: Option<i32>,
    pub source_end_char: Option<i32>,
    pub full_audio_start_ms: Option<i32>,
    pub full_audio_end_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KeyPoint {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub key_point_number: i32,
    pub title: Option<String>,
    pub segment_start: i32,
    pub segment_end: i32,
    pub full_audio_start_ms: Option<i32>,
    pub full_audio_end_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Summary {
    pub id: Uuid,
    pub chapter_id: Option<Uuid>,
    pub key_point_id: Option<Uuid>,
    pub text: Option<String>,
    pub word_count: Option<i32>,
    pub status: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AudioFile {
    pub id: Uuid,
    pub chapter_id: Option<Uuid>,
    pub segment_id: Option<Uuid>,
    pub summary_id: Option<Uuid>,
    pub episode_id: Option<Uuid>,
    pub audio_type: String,
    pub status: String,
    pub file_id: Option<Uuid>,
    pub file_path: Option<String>,
    pub duration_ms: Option<i32>,
    pub file_size_bytes: Option<i32>,
    pub voice_id: Option<Uuid>,
    pub provider: Option<String>,
    pub retry_count: i32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
