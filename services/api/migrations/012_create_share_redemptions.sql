-- Create share_redemptions table
-- Note: user_id references users in Auth Service database (no FK constraint)
CREATE TABLE share_redemptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    share_id UUID NOT NULL REFERENCES shares(id),
    user_id UUID NOT NULL,
    redeemed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(share_id, user_id)
);

CREATE INDEX idx_share_redemptions_share ON share_redemptions(share_id);
CREATE INDEX idx_share_redemptions_user ON share_redemptions(user_id);
