-- Fix chapters.project_id FK to cascade on project delete
ALTER TABLE chapters DROP CONSTRAINT chapters_project_id_fkey;
ALTER TABLE chapters ADD CONSTRAINT chapters_project_id_fkey
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;

-- Also cascade playback_progress
ALTER TABLE playback_progress DROP CONSTRAINT IF EXISTS playback_progress_project_id_fkey;
ALTER TABLE playback_progress ADD CONSTRAINT playback_progress_project_id_fkey
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;
