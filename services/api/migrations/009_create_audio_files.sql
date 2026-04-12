-- Create audio_files table
CREATE TABLE audio_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID REFERENCES chapters(id) ON DELETE CASCADE,
    segment_id UUID REFERENCES segments(id) ON DELETE CASCADE,
    summary_id UUID REFERENCES summaries(id) ON DELETE CASCADE,
    episode_id UUID REFERENCES podcast_episodes(id) ON DELETE CASCADE,
    audio_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    file_id UUID,
    file_path VARCHAR(500),
    duration_ms INTEGER,
    file_size_bytes INTEGER,
    voice_id UUID REFERENCES voices(id),
    provider VARCHAR(50),
    retry_count INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    queued_at TIMESTAMP WITH TIME ZONE,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
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
