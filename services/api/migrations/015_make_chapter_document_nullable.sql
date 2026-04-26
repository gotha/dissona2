-- Make document_id and chapter_number nullable on chapters table
-- Sample chapters don't have associated documents or traditional numbering
ALTER TABLE chapters ALTER COLUMN document_id DROP NOT NULL;
ALTER TABLE chapters ALTER COLUMN chapter_number DROP NOT NULL;
