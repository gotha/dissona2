-- Add has_completed_first_upload flag for onboarding flow
ALTER TABLE users ADD COLUMN IF NOT EXISTS has_completed_first_upload BOOLEAN NOT NULL DEFAULT false;
