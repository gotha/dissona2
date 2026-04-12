-- Create voices table
CREATE TABLE voices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider VARCHAR(50) NOT NULL,
    provider_voice_id VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    language VARCHAR(10) NOT NULL,
    gender VARCHAR(20),
    sample_url VARCHAR(500),
    is_active BOOLEAN DEFAULT TRUE,
    tier_required VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_voice_id)
);

-- Seed default voices (Kokoro)
INSERT INTO voices (provider, provider_voice_id, name, language, gender) VALUES
('kokoro', 'af_bella', 'Bella', 'en-US', 'female'),
('kokoro', 'af_sarah', 'Sarah', 'en-US', 'female'),
('kokoro', 'am_adam', 'Adam', 'en-US', 'male'),
('kokoro', 'am_michael', 'Michael', 'en-US', 'male'),
('kokoro', 'bf_emma', 'Emma', 'en-GB', 'female'),
('kokoro', 'bm_george', 'George', 'en-GB', 'male');
