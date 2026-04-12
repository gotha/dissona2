# Database Specification

## Overview

Two separate PostgreSQL 16 databases:

| Database | Service | Port | Purpose |
|----------|---------|------|---------|
| `disona_auth` | Auth Service | 5432 | Users, preferences |
| `disona` | API Service | 5433 | Projects, content, audio |

## Separation Rationale

- **Auth Service** owns user identity (email, name, OAuth)
- **API Service** stores `user_id` as a UUID reference (no FK constraint)
- JWT tokens contain user info, so API doesn't need to query auth database
- Each service manages its own migrations
| **Readers** | API Service, Auth Service |

**See also:** [Database ER Diagram](./database-diagram.md)

---

## Design Principles

1. **Single Writer** — Only API Service writes to database
2. **UUID Primary Keys** — No sequential IDs exposed
3. **Soft Deletes** — `deleted_at` for important tables
4. **Audit Fields** — `created_at`, `updated_at` on all tables

---

## Terminology

| Term | Definition |
|------|------------|
| **Project** | Container for creating audio content |
| **Document** | Source material (uploaded PDF, article, text) |
| **Chapter** | Major division of a document |
| **Segment** | Smallest unit within a chapter |
| **Key Point** | Group of segments around a topic |
| **Summary** | LLM-generated summary (for chapter or key point) |
| **Audio File** | Generated audio (for chapter, segment, summary, or episode) |
| **Podcast Episode** | Generated podcast script + audio |

---

## Schema Overview

```
users
  └── projects
        │
        ├── documents (sources)
        │     └── chapters
        │           ├── segments
        │           ├── key_points
        │           │     └── summaries (key point)
        │           ├── summaries (chapter)
        │           └── audio_files (chapter/segment audio)
        │
        ├── audio_files (audiobook output - optional)
        │     └── chapter summaries, key point summaries, full narration
        │
        └── podcast_episodes (podcast output - optional)
              └── audio_files (episode audio)

  └── playback_progress
  └── shares
        └── share_redemptions
  └── user_preferences

Note: A project can have BOTH audiobook audio AND podcast episodes.
      They are generated independently from the same source documents.
```

---

## Tables

### users

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255),
    avatar_url VARCHAR(500),
    google_id VARCHAR(255) UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Note: For MVP, all users have full unrestricted access.
-- Subscriptions, tiers, and quotas will be added later.
```

### projects

```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    cover_image_path VARCHAR(500),

    -- Voice (NULL = use user's default)
    voice_id UUID REFERENCES voices(id),

    -- Document processing status
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    -- draft → uploading → processing → ready → failed
    substatus VARCHAR(100),

    -- Progress counters
    documents_total INTEGER DEFAULT 0,
    documents_processed INTEGER DEFAULT 0,
    chapters_total INTEGER DEFAULT 0,
    chapters_ready INTEGER DEFAULT 0,

    -- Output generation status (both can be generated)
    audiobook_status VARCHAR(50),  -- NULL, generating, ready, failed
    podcast_status VARCHAR(50),     -- NULL, generating, ready, failed

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_projects_user ON projects(user_id);
CREATE INDEX idx_projects_status ON projects(user_id, status);
```

### documents

```sql
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    title VARCHAR(500),
    author VARCHAR(255),
    source_file_path VARCHAR(500),
    source_file_size INTEGER,
    source_type VARCHAR(50),  -- pdf, article, text
    page_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending → uploading → processing → processed → failed
    substatus VARCHAR(100),
    detection_method VARCHAR(50),  -- toc, headings, patterns, semantic
    extracted_chapters_count INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, position)
);

CREATE INDEX idx_documents_project ON documents(project_id);
```

### chapters

```sql
CREATE TABLE chapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id),  -- denormalized
    chapter_number INTEGER NOT NULL,
    title VARCHAR(500),
    source_text TEXT,
    word_count INTEGER,
    estimated_duration_min INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending → analyzing → analyzed → generating_audio → ready → failed
    substatus VARCHAR(100),
    segments_total INTEGER DEFAULT 0,
    key_points_total INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, chapter_number)
);

CREATE INDEX idx_chapters_project ON chapters(project_id);
CREATE INDEX idx_chapters_document ON chapters(document_id);
CREATE INDEX idx_chapters_status ON chapters(project_id, status);
```

### segments

```sql
CREATE TABLE segments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    segment_number INTEGER NOT NULL,
    text TEXT NOT NULL,
    word_count INTEGER,
    source_start_char INTEGER,
    source_end_char INTEGER,
    full_audio_start_ms INTEGER,
    full_audio_end_ms INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(chapter_id, segment_number)
);

CREATE INDEX idx_segments_chapter ON segments(chapter_id);
```

### key_points

```sql
CREATE TABLE key_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    key_point_number INTEGER NOT NULL,
    title VARCHAR(500),
    segment_start INTEGER NOT NULL,
    segment_end INTEGER NOT NULL,
    full_audio_start_ms INTEGER,
    full_audio_end_ms INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(chapter_id, key_point_number)
);

CREATE INDEX idx_key_points_chapter ON key_points(chapter_id);
```

### summaries

```sql
CREATE TABLE summaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID REFERENCES chapters(id) ON DELETE CASCADE,
    key_point_id UUID REFERENCES key_points(id) ON DELETE CASCADE,
    text TEXT,
    word_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending → generating → ready → failed
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT summary_has_one_parent CHECK (
        (chapter_id IS NOT NULL)::int + (key_point_id IS NOT NULL)::int = 1
    )
);

CREATE INDEX idx_summaries_chapter ON summaries(chapter_id);
CREATE INDEX idx_summaries_key_point ON summaries(key_point_id);
```

### podcast_episodes

```sql
CREATE TABLE podcast_episodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL,
    title VARCHAR(500),
    description TEXT,
    script TEXT,
    word_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending → generating_script → script_ready → generating_audio → ready → failed
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, episode_number)
);

CREATE INDEX idx_podcast_episodes_project ON podcast_episodes(project_id);
```

### audio_files

```sql
CREATE TABLE audio_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID REFERENCES chapters(id) ON DELETE CASCADE,
    segment_id UUID REFERENCES segments(id) ON DELETE CASCADE,
    summary_id UUID REFERENCES summaries(id) ON DELETE CASCADE,
    episode_id UUID REFERENCES podcast_episodes(id) ON DELETE CASCADE,
    audio_type VARCHAR(50) NOT NULL,
    -- 'chapter_full', 'segment_full', 'chapter_summary', 'key_point_summary', 'podcast_episode'
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending → queued → generating → ready → failed
    file_id UUID,
    file_path VARCHAR(500),
    duration_ms INTEGER,
    file_size_bytes INTEGER,
    voice_id UUID REFERENCES voices(id),
    provider VARCHAR(50),
    retry_count INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    queued_at TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT audio_has_one_parent CHECK (
        (chapter_id IS NOT NULL)::int +
        (segment_id IS NOT NULL)::int +
        (summary_id IS NOT NULL)::int +
        (episode_id IS NOT NULL)::int = 1
    )
);

CREATE INDEX idx_audio_files_chapter ON audio_files(chapter_id);
CREATE INDEX idx_audio_files_summary ON audio_files(summary_id);
CREATE INDEX idx_audio_files_episode ON audio_files(episode_id);
CREATE INDEX idx_audio_files_status ON audio_files(status);
```

### voices

```sql
CREATE TABLE voices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider VARCHAR(50) NOT NULL,
    provider_voice_id VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    language VARCHAR(10) NOT NULL,
    gender VARCHAR(20),
    sample_url VARCHAR(500),
    is_active BOOLEAN DEFAULT TRUE,
    tier_required VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_voice_id)
);
```

### playback_progress

```sql
CREATE TABLE playback_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    current_chapter_id UUID REFERENCES chapters(id),
    current_audio_type VARCHAR(50),
    position_ms INTEGER DEFAULT 0,
    listening_mode VARCHAR(20) DEFAULT 'blitz',
    chapters_completed JSONB DEFAULT '[]',
    completed BOOLEAN DEFAULT FALSE,
    completed_at TIMESTAMP,
    last_played_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, project_id)
);

CREATE INDEX idx_playback_user ON playback_progress(user_id);
```

### shares

```sql
CREATE TABLE shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id),
    project_id UUID REFERENCES projects(id),
    chapter_id UUID REFERENCES chapters(id),
    segment_id UUID REFERENCES segments(id),
    token VARCHAR(100) NOT NULL UNIQUE,
    recipient_email VARCHAR(255),
    personal_message TEXT,
    expires_at TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT share_has_one_target CHECK (
        (project_id IS NOT NULL)::int +
        (chapter_id IS NOT NULL)::int +
        (segment_id IS NOT NULL)::int = 1
    )
);

CREATE INDEX idx_shares_owner ON shares(owner_id);
CREATE INDEX idx_shares_token ON shares(token);
```

### share_redemptions

```sql
CREATE TABLE share_redemptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    share_id UUID NOT NULL REFERENCES shares(id),
    user_id UUID NOT NULL REFERENCES users(id),
    redeemed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(share_id, user_id)
);
```

### user_preferences

```sql
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    default_voice_id UUID REFERENCES voices(id),
    playback_speed DECIMAL(3,2) DEFAULT 1.0,
    default_mode VARCHAR(20) DEFAULT 'blitz',
    auto_play_next BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

---

## Migrations

Use `sqlx` migrations in API Service:

```
migrations/
├── 001_create_users.sql
├── 002_create_projects.sql
├── 003_create_documents.sql
├── 004_create_chapters.sql
├── 005_create_segments.sql
├── 006_create_key_points.sql
├── 007_create_summaries.sql
├── 008_create_podcast_episodes.sql
├── 009_create_voices.sql
├── 010_create_audio_files.sql
├── 011_create_playback_progress.sql
├── 012_create_shares.sql
├── 013_create_user_preferences.sql
└── 014_seed_voices.sql
```
