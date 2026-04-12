-- Create playback_progress table
-- Note: user_id references users in Auth Service database (no FK constraint)
CREATE TABLE playback_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    current_chapter_id UUID REFERENCES chapters(id),
    current_audio_type VARCHAR(50),
    position_ms INTEGER DEFAULT 0,
    listening_mode VARCHAR(20) DEFAULT 'blitz',
    chapters_completed JSONB DEFAULT '[]',
    completed BOOLEAN DEFAULT FALSE,
    completed_at TIMESTAMP WITH TIME ZONE,
    last_played_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, project_id)
);

CREATE INDEX idx_playback_user ON playback_progress(user_id);
