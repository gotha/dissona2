# Story 2.2: Document Processing & Chapter Detection

**Epic:** E2 - Content Upload & Processing
**Priority:** P0
**Status:** Ready for Dev
**Story Points:** 8
**Created:** 2026-04-26

---

## User Story

As a **user**,
I want to **have my PDF automatically parsed into chapters**,
So that I can **navigate the content by chapter**.

---

## Acceptance Criteria

### AC1: PDF Parsing
- **Given** a PDF has been uploaded
- **When** the PDF Worker receives the `pdf.parse` job
- **Then** it downloads the PDF from S3
- **And** extracts text from all pages
- **And** OCR is used for image-only pages (if enabled)

### AC2: Chapter Detection
- **Given** text has been extracted
- **When** chapter detection runs
- **Then** chapters are detected using: TOC → heading patterns → text patterns
- **And** if no chapters detected, content is treated as a single chapter
- **And** each chapter has title, text, word count

### AC3: Database Updates
- **Given** chapters have been detected
- **When** processing completes
- **Then** document record updated: status=`processed`, page_count, detection_method
- **And** chapter records created with title, source_text, word_count
- **And** project status updated to `ready`

### AC4: Status Progression
- **Given** the PDF is being processed
- **When** each substep completes
- **Then** document substatus updates: `parsing` → `detecting_chapters` → `analyzing`

### AC5: Error Handling
- **Given** processing fails
- **When** an error occurs
- **Then** document status set to `failed` with error_code and error_message
- **And** project status set to `failed`

---

## Technical Tasks

### PDF Worker (Python)
1. Connect to NATS and pull `jobs.pdf.parse` messages
2. Download PDF from S3 using boto3
3. Extract text with PyMuPDF (fitz)
4. Detect chapters (TOC, headings, patterns, single fallback)
5. Connect to PostgreSQL API database directly
6. Update document record (status, page_count, detection_method)
7. Insert chapter records
8. Update project status to `ready` or `failed`
9. Publish `events.pdf.completed` to NATS

### Infrastructure
10. Add `DATABASE_URL` to PDF Worker config
11. Add `psycopg2` to requirements.txt

---

## Dependencies
- Story 2.1 (PDF uploaded to S3)
- NATS streams created
- PostgreSQL API database accessible from worker
