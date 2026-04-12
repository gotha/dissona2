-- Create chapters table
CREATE TABLE chapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id),
    chapter_number INTEGER NOT NULL,
    title VARCHAR(500),
    source_text TEXT,
    word_count INTEGER,
    estimated_duration_min INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    substatus VARCHAR(100),
    segments_total INTEGER DEFAULT 0,
    key_points_total INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, chapter_number)
);

CREATE INDEX idx_chapters_project ON chapters(project_id);
CREATE INDEX idx_chapters_document ON chapters(document_id);
CREATE INDEX idx_chapters_status ON chapters(project_id, status);
