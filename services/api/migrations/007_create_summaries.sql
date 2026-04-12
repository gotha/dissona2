-- Create summaries table
CREATE TABLE summaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chapter_id UUID REFERENCES chapters(id) ON DELETE CASCADE,
    key_point_id UUID REFERENCES key_points(id) ON DELETE CASCADE,
    text TEXT,
    word_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT summary_has_one_parent CHECK (
        (chapter_id IS NOT NULL)::int + (key_point_id IS NOT NULL)::int = 1
    )
);

CREATE INDEX idx_summaries_chapter ON summaries(chapter_id);
CREATE INDEX idx_summaries_key_point ON summaries(key_point_id);
