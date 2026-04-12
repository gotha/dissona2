# API Service Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Rust |
| **Framework** | Actix-web |
| **Port** | 8080 |
| **Database** | PostgreSQL (exclusive writer) |
| **Message Queue** | NATS JetStream (publish + subscribe) |

---

## Responsibilities

### Primary Functions

1. **REST API** — Serve all frontend requests
2. **Database Management** — Only service that writes to PostgreSQL
3. **Event Processing** — Consume completion events from workers
4. **Job Publishing** — Trigger worker jobs via NATS

### What This Service Does NOT Do

- ❌ Authenticate users (Auth Service does this)
- ❌ Parse PDFs (PDF Worker does this)
- ❌ Generate summaries (LLM Worker does this)
- ❌ Generate audio (TTS Worker does this)
- ❌ Host ML models (Ollama does this)

---

## API Endpoints

### Books

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/books` | List user's books |
| `POST` | `/api/books` | Upload new book (PDF) |
| `GET` | `/api/books/{id}` | Get book details |
| `DELETE` | `/api/books/{id}` | Delete book |
| `GET` | `/api/books/{id}/status` | Get processing status |
| `POST` | `/api/books/{id}/generate` | Trigger audio generation |
| `GET` | `/api/books/{id}/manifest` | Get audio manifest (file IDs, durations) |

### Chapters

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/books/{id}/chapters` | List chapters |
| `GET` | `/api/chapters/{id}` | Get chapter details |
| `GET` | `/api/chapters/{id}/audio` | Get chapter audio URLs |

### Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/users/me` | Get current user profile |
| `PATCH` | `/api/users/me` | Update profile |
| `GET` | `/api/users/me/quota` | Get usage quota |

### Playback

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/playback/progress` | Get all book progress |
| `PUT` | `/api/playback/progress/{book_id}` | Update playback position |
| `GET` | `/api/playback/queue` | Get playback queue |
| `PUT` | `/api/playback/queue` | Update queue |

### Sharing

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/shares` | Create share |
| `GET` | `/api/shares` | List shares created by user |
| `DELETE` | `/api/shares/{id}` | Revoke share |
| `GET` | `/api/shared-with-me` | List shares received |
| `POST` | `/api/shares/{token}/redeem` | Redeem share (public) |

---

## Event Consumers

The API Service runs internal workers that consume NATS events:

### PDF Completion Handler

**Subject:** `pdf.completed`

```rust
async fn handle_pdf_completed(event: PdfCompletedEvent) {
    // 1. Create/update book record
    // 2. Create chapter records
    // 3. Update book status to "analyzed"
    // 4. Publish llm.segment jobs for each chapter
}
```

### LLM Completion Handler

**Subject:** `llm.segment.completed`, `llm.summarize.completed`

```rust
async fn handle_llm_completed(event: LlmCompletedEvent) {
    // 1. Store key points / summaries in database
    // 2. Update chapter status
    // 3. Check if all LLM work complete
    // 4. Update book status if complete
}
```

### TTS Completion Handler

**Subject:** `tts.completed`

```rust
async fn handle_tts_completed(event: TtsCompletedEvent) {
    // 1. Update chapter_audio record (file_id, duration)
    // 2. Update chapter status
    // 3. Update book progress counts
    // 4. Check if book fully generated
}
```

---

## Job Publishers

| Job | Subject | Trigger |
|-----|---------|---------|
| PDF Parse | `pdf.parse` | Book upload |
| LLM Segment | `llm.segment` | PDF complete |
| LLM Summarize L1 | `llm.summarize.l1` | Segment complete |
| LLM Summarize L2 | `llm.summarize.l2` | Segment complete |
| TTS Generate | `tts.generate` | User clicks "Generate" |

---

## Database Tables Owned

| Table | Description |
|-------|-------------|
| `users` | User accounts |
| `subscriptions` | Subscription status |
| `books` | Book metadata and status |
| `chapters` | Chapter metadata |
| `chapter_key_points` | Key points per chapter |
| `chapter_audio` | Audio file references and status |
| `playback_progress` | Per-book playback position |
| `shares` | Share records |
| `share_redemptions` | Who redeemed which shares |

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection | required |
| `NATS_URL` | NATS server | `nats://localhost:4222` |
| `S3_ENDPOINT` | Object storage | `http://localhost:9000` |
| `S3_BUCKET_AUDIO` | Audio bucket | `disona-audio` |
| `S3_BUCKET_UPLOADS` | Upload bucket | `disona-uploads` |

---

## Error Handling

| Error Type | HTTP Status | Response |
|------------|-------------|----------|
| Not found | 404 | `{"error": "Book not found"}` |
| Unauthorized | 401 | `{"error": "Invalid token"}` |
| Forbidden | 403 | `{"error": "Not your book"}` |
| Quota exceeded | 402 | `{"error": "Quota exceeded", "remaining": 0}` |
| Validation | 400 | `{"error": "Invalid input", "details": [...]}` |
| Server error | 500 | `{"error": "Internal error"}` |
