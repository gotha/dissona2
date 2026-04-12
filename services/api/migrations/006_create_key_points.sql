-- Create key_points table
CREATE TABLE key_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    key_point_number INTEGER NOT NULL,
    title VARCHAR(500),
    segment_start INTEGER NOT NULL,
    segment_end INTEGER NOT NULL,
    full_audio_start_ms INTEGER,
    full_audio_end_ms INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(chapter_id, key_point_number)
);

CREATE INDEX idx_key_points_chapter ON key_points(chapter_id);
