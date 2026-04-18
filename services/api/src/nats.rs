// NATS JetStream utilities for publishing jobs and subscribing to events

use async_nats::jetstream::{self, Context};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job subjects
pub mod subjects {
    pub const PDF_PARSE: &str = "jobs.pdf.parse";
    pub const LLM_SEGMENT: &str = "jobs.llm.segment";
    pub const LLM_SUMMARIZE: &str = "jobs.llm.summarize";
    pub const LLM_KEYPOINTS: &str = "jobs.llm.keypoints";
    pub const TTS_GENERATE: &str = "jobs.tts.generate";

    pub const EVENTS_PDF: &str = "events.pdf.*";
    pub const EVENTS_LLM: &str = "events.llm.*";
    pub const EVENTS_TTS: &str = "events.tts.*";
}

/// PDF parsing job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfParseJob {
    pub job_id: Uuid,
    pub document_id: Uuid,
    pub project_id: Uuid,
    pub file_path: String,
}

/// LLM segmentation job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSegmentJob {
    pub job_id: Uuid,
    pub chapter_id: Uuid,
    pub text: String,
}

/// LLM summarization job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSummarizeJob {
    pub job_id: Uuid,
    pub chapter_id: Uuid,
    pub key_point_id: Option<Uuid>,
    pub text: String,
    pub summary_type: String, // "chapter" or "key_point"
}

/// TTS generation job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsGenerateJob {
    pub job_id: Uuid,
    pub audio_file_id: Uuid,
    pub text: String,
    pub voice_id: Option<Uuid>,
}

/// Event: PDF parsing completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfParsedEvent {
    pub document_id: Uuid,
    pub chapters: Vec<ParsedChapter>,
    pub detection_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedChapter {
    pub number: i32,
    pub title: Option<String>,
    pub text: String,
    pub word_count: i32,
}

/// Event: LLM task completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmCompletedEvent {
    pub job_id: Uuid,
    pub task_type: String,
    pub chapter_id: Option<Uuid>,
    pub key_point_id: Option<Uuid>,
    pub result: serde_json::Value,
}

/// Event: TTS generation completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsCompletedEvent {
    pub audio_file_id: Uuid,
    pub file_path: String,
    pub duration_ms: i32,
    pub file_size_bytes: i32,
}

/// Initialize JetStream streams
pub async fn init_streams(js: &Context) -> Result<(), async_nats::Error> {
    // Jobs stream
    js.get_or_create_stream(jetstream::stream::Config {
        name: "JOBS".to_string(),
        subjects: vec![
            subjects::PDF_PARSE.to_string(),
            subjects::LLM_SEGMENT.to_string(),
            subjects::LLM_SUMMARIZE.to_string(),
            subjects::LLM_KEYPOINTS.to_string(),
            subjects::TTS_GENERATE.to_string(),
        ],
        retention: jetstream::stream::RetentionPolicy::WorkQueue,
        ..Default::default()
    })
    .await?;

    // Events stream
    js.get_or_create_stream(jetstream::stream::Config {
        name: "EVENTS".to_string(),
        subjects: vec![
            "events.>".to_string(),
        ],
        retention: jetstream::stream::RetentionPolicy::Limits,
        max_age: std::time::Duration::from_secs(7 * 24 * 60 * 60), // 7 days
        ..Default::default()
    })
    .await?;

    Ok(())
}

/// Publish a job to NATS
pub async fn publish_job<T: Serialize>(
    js: &Context,
    subject: impl Into<String>,
    job: &T,
) -> Result<(), async_nats::Error> {
    let subject = subject.into();
    let payload = serde_json::to_vec(job).expect("Failed to serialize job");
    js.publish(subject, payload.into()).await?.await?;
    Ok(())
}
