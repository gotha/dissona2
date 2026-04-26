# Story 2.1: PDF Upload with Progress

**Epic:** E2 - Content Upload & Processing
**Priority:** P0
**Status:** Ready for Dev
**Story Points:** 8
**Created:** 2026-04-26

---

## User Story

As a **user**,
I want to **upload a PDF file and see upload progress**,
So that I can **know my file is being transferred successfully**.

---

## Acceptance Criteria

### AC1: Upload Button in Library
- **Given** I am on the library page
- **When** I tap "New Project" / "Upload a file"
- **Then** a native file picker opens accepting `.pdf` files

### AC2: Upload Progress
- **Given** I have selected a PDF file
- **When** the upload is in progress
- **Then** I see a progress bar with percentage
- **And** I can cancel the upload

### AC3: File Size Limit
- **Given** I select a file larger than 100MB
- **When** the upload is attempted
- **Then** I see an error message "File too large (max 100MB)"

### AC4: S3 Storage
- **Given** my upload completes
- **When** the file is stored
- **Then** it is saved to MinIO/S3 bucket `dissona-uploads` at `{project_id}/original.pdf`

### AC5: Project Creation
- **Given** upload succeeds
- **When** the project is created
- **Then** a project record is created with status `uploading` → `processing`
- **And** a document record is created linked to the project
- **And** a `pdf.parse` job is published to NATS

---

## Technical Tasks

### Backend (API Service)
1. Create S3 client module (`src/s3.rs`) for upload/download/presigned URLs
2. Update `create_project` handler to accept multipart form data with PDF
3. Upload PDF to S3 bucket `dissona-uploads/{project_id}/original.pdf`
4. Create project + document records in DB
5. Publish `PdfParseJob` to NATS `jobs.pdf.parse`
6. Initialize NATS streams on API startup
7. Create MinIO buckets on startup if they don't exist

### Frontend
8. Update Library page "New Project" button to trigger file picker
9. Create upload modal/overlay with progress bar
10. POST multipart form to `/api/projects` with file
11. Show upload progress via XMLHttpRequest or axios progress
12. Navigate to project detail page on completion

---

## Dependencies
- MinIO running (docker-compose)
- NATS JetStream running (docker-compose)
- Auth working (Epic 1)
