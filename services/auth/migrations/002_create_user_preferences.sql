-- Create user_preferences table
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    default_voice_id UUID,  -- References voices in API database (no FK)
    playback_speed DECIMAL(3,2) DEFAULT 1.0,
    default_mode VARCHAR(20) DEFAULT 'blitz',
    auto_play_next BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
