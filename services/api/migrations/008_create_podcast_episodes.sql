-- Create podcast_episodes table
CREATE TABLE podcast_episodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL,
    title VARCHAR(500),
    description TEXT,
    script TEXT,
    word_count INTEGER,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_code VARCHAR(50),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, episode_number)
);

CREATE INDEX idx_podcast_episodes_project ON podcast_episodes(project_id);
