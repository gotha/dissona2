-- Create projects table
-- Note: user_id references users in Auth Service database (no FK constraint)
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    cover_image_path VARCHAR(500),
    
    -- Voice (NULL = use user's default)
    voice_id UUID REFERENCES voices(id),
    
    -- Document processing status
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    substatus VARCHAR(100),
    
    -- Progress counters
    documents_total INTEGER DEFAULT 0,
    documents_processed INTEGER DEFAULT 0,
    chapters_total INTEGER DEFAULT 0,
    chapters_ready INTEGER DEFAULT 0,
    
    -- Output generation status (both can be generated)
    audiobook_status VARCHAR(50),
    podcast_status VARCHAR(50),
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_projects_user ON projects(user_id);
CREATE INDEX idx_projects_status ON projects(user_id, status);
