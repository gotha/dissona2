-- Create segments table
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
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(chapter_id, segment_number)
);

CREATE INDEX idx_segments_chapter ON segments(chapter_id);
