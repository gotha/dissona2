-- Add missing columns for sample content feature

-- Add is_sample column to projects table
ALTER TABLE projects ADD COLUMN IF NOT EXISTS is_sample BOOLEAN NOT NULL DEFAULT false;

-- Add summary column to chapters table  
ALTER TABLE chapters ADD COLUMN IF NOT EXISTS summary TEXT;

-- Add chapter_order column to chapters table
ALTER TABLE chapters ADD COLUMN IF NOT EXISTS chapter_order INTEGER;

-- Add audio_status column to chapters table
ALTER TABLE chapters ADD COLUMN IF NOT EXISTS audio_status VARCHAR(50) DEFAULT 'pending';

-- Create index for finding sample projects
CREATE INDEX IF NOT EXISTS idx_projects_is_sample ON projects(is_sample) WHERE is_sample = true;
