-- Create shares table
-- Note: owner_id references users in Auth Service database (no FK constraint)
CREATE TABLE shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    project_id UUID REFERENCES projects(id),
    chapter_id UUID REFERENCES chapters(id),
    segment_id UUID REFERENCES segments(id),
    token VARCHAR(100) NOT NULL UNIQUE,
    recipient_email VARCHAR(255),
    personal_message TEXT,
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT share_has_one_target CHECK (
        (project_id IS NOT NULL)::int +
        (chapter_id IS NOT NULL)::int +
        (segment_id IS NOT NULL)::int = 1
    )
);

CREATE INDEX idx_shares_owner ON shares(owner_id);
CREATE INDEX idx_shares_token ON shares(token);
