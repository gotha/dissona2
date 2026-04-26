# Story 2.4: Processing Status Polling

**Epic:** E2 - Content Upload & Processing
**Priority:** P0
**Status:** Ready for Dev
**Story Points:** 5
**Created:** 2026-04-26

---

## User Story

As a **user**,
I want to **see real-time processing status updates**,
So that I can **know what's happening with my document**.

---

## Acceptance Criteria

### AC1: Status Polling
- **Given** my document is processing
- **When** I am on the library or project detail page
- **Then** status updates every 3 seconds automatically

### AC2: Status Display
- **Given** processing is ongoing
- **When** the status changes
- **Then** I see the current substep: "Parsing PDF...", "Detecting chapters...", "Analyzing..."

### AC3: Completion
- **Given** processing succeeds
- **When** status becomes `ready`
- **Then** the project card in library shows chapter count
- **And** if on project detail page, chapters are displayed

### AC4: Failure
- **Given** processing fails
- **When** status becomes `failed`
- **Then** I see an error message with the failure reason
- **And** a "Retry" option is available

---

## Technical Tasks

### Frontend
1. Add polling hook (`useProjectPolling`) with 3s interval
2. Update Library ProjectCard to show processing status inline
3. Update Project detail page to poll and display status
4. Show chapter list when project becomes `ready`
5. Handle error states with retry button

### Backend (API)
6. Ensure `GET /api/projects/{id}` returns substatus field
7. Ensure `GET /api/projects` returns status for all projects

---

## Dependencies
- Story 2.1 (project created)
- Story 2.2 (status updates in DB)
