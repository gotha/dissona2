-- Create documents table
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    title VARCHAR(500),
    author VARCHAR(255),
    source_file_path VARCHAR(500),
    source_file_size INTEGER,
    source_type VARCHAR(50),
    page_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    substatus VARCHAR(100),
    detection_method VARCHAR(50),
    extracted_chapters_count INTEGER DEFAULT 0,
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, position)
);

CREATE INDEX idx_documents_project ON documents(project_id);
