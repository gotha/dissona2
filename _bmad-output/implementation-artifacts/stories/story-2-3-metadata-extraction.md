# Story 2.3: Auto-Populate Project Metadata

**Epic:** E2 - Content Upload & Processing
**Priority:** P0
**Status:** Ready for Dev
**Story Points:** 3
**Created:** 2026-04-26

---

## User Story

As a **user**,
I want to **have project title and author auto-extracted**,
So that I can **skip manual data entry**.

---

## Acceptance Criteria

### AC1: Title Extraction
- **Given** a PDF is being processed
- **When** metadata extraction runs
- **Then** title is extracted from PDF metadata
- **And** if no PDF metadata title, filename (without .pdf) is used

### AC2: Author Extraction
- **Given** a PDF has metadata
- **When** author field exists
- **Then** author is stored on the document record

### AC3: Project Title Update
- **Given** title has been extracted
- **When** processing completes
- **Then** the project title is updated (if it was auto-generated from filename)
- **And** the document title and author fields are populated

---

## Technical Tasks

### PDF Worker
1. Extract metadata from PDF using `fitz.Document.metadata`
2. Update document record with title, author from metadata
3. Update project title if it was set from filename

---

## Dependencies
- Story 2.2 (integrated into chapter detection pipeline)
