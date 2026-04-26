# Story 2.5: Multi-Document Project (Deferred)

**Epic:** E2 - Content Upload & Processing
**Priority:** P1 (Future)
**Status:** Deferred
**Story Points:** 5
**Created:** 2026-04-26

---

## User Story

As a **user**,
I want to **add multiple PDFs to a single project**,
So that I can **combine related documents into one audiobook**.

---

## Acceptance Criteria

### AC1: Add Another File
- **Given** I have uploaded one PDF and processing is ongoing
- **When** I tap "Add another file"
- **Then** I can select additional PDFs

### AC2: Combined Chapters
- **Given** multiple documents are processed
- **When** all complete
- **Then** chapters from all documents are combined in order

### AC3: Lock After Ready
- **Given** processing succeeds
- **When** project status is `ready`
- **Then** I cannot add more files

---

## Notes

This story is deferred to a future sprint. The current implementation supports
single-document projects. The `documents` table and API already support
multiple documents per project, so this is primarily a frontend concern.

---

## Dependencies
- Story 2.1 (upload flow)
- Story 2.2 (processing pipeline)
