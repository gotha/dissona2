# Story 1.4: Sample Content for New Users

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 5  
**Created:** 2026-04-12

---

## User Story

As a **new user**,  
I want to **try sample content without uploading anything**,  
So that I can **understand how Disona works before committing**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR55 | Provide sample content for new users |
| FR56 | Sample demonstrates all listening modes |

---

## Acceptance Criteria

### AC1: Sample Content Visible

- **Given** I just signed up and have no projects
- **When** I land on the empty library
- **Then** I see a "Try a Sample" option prominently displayed
- **And** the sample is described (e.g., "5-minute TED talk")

### AC2: One-Tap Access

- **Given** I see the sample option
- **When** I tap "Try a Sample"
- **Then** the sample project is added to my library
- **And** I am taken to the project detail view

### AC3: Pre-Generated Audio

- **Given** I access the sample project
- **When** I view the project
- **Then** audio is already generated (no waiting)
- **And** I can start listening immediately

### AC4: All Modes Available

- **Given** I am playing the sample
- **When** I view the player
- **Then** I can switch between Blitz and Full modes
- **And** Go Deep works correctly
- **And** all player controls work

### AC5: Sample Marked as Sample

- **Given** the sample is in my library
- **When** I view the library
- **Then** it's clearly marked as "Sample"
- **And** I can delete it like any other project

### AC6: Not Counted Against Quota

- **Given** I use the sample content
- **When** I check my quota
- **Then** the sample does not count against my limits

---

## Technical Design

### Sample Content Strategy

**Option A: System Sample (Shared)**
- Single sample project in database
- All users reference same content
- No duplication, minimal storage

**Option B: User Copy (Recommended)**
- Copy sample to user's library
- User has full control (delete, etc.)
- Slightly more storage, better UX

**Decision:** Option B - Create a copy for each user

### Sample Content

**Proposed Sample:** "The Power of Vulnerability" by Brené Brown (TED Talk)
- Duration: ~20 minutes
- Clear chapters/sections
- Engaging content
- Demonstrates value proposition

**Alternative:** Public domain content or original content

### Components to Create

| Component | File | Description |
|-----------|------|-------------|
| Sample Seeder | `services/api/src/sample.rs` | Creates sample for user |
| Empty State | `frontend/src/components/EmptyLibrary.tsx` | Sample CTA |
| Sample Badge | `frontend/src/components/ProjectCard.tsx` | "Sample" label |

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.4.1** Create sample project data (JSON fixture)
- [ ] **1.4.2** Create sample audio files (L1, L2, Full)
- [ ] **1.4.3** Create `POST /api/samples/try` endpoint
- [ ] **1.4.4** Add `is_sample` flag to projects table
- [ ] **1.4.5** Exclude samples from quota calculations
- [ ] **1.4.6** Unit tests for sample creation

### Frontend Tasks

- [ ] **1.4.7** Create EmptyLibrary component with sample CTA
- [ ] **1.4.8** Add "Sample" badge to ProjectCard
- [ ] **1.4.9** Handle sample loading state
- [ ] **1.4.10** Unit tests for EmptyLibrary
- [ ] **1.4.11** E2E test for sample flow

### Content Tasks

- [ ] **1.4.12** Source or create sample content (legally cleared)
- [ ] **1.4.13** Generate L1/L2 summaries for sample
- [ ] **1.4.14** Generate TTS audio for sample

---

## API Contracts

### Endpoint: POST /api/samples/try

**Description:** Creates a copy of sample content for user

**Response:** HTTP 201
```json
{
  "project": {
    "id": "uuid",
    "title": "The Power of Vulnerability (Sample)",
    "is_sample": true,
    "chapters": [...],
    "audiobook_status": "ready"
  }
}
```

---

## Empty Library UI

```
┌─────────────────────────────────────────────────────┐
│  Disona                                    [Avatar] │
├─────────────────────────────────────────────────────┤
│                                                     │
│                      📚                             │
│                                                     │
│           Your library is empty                     │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  ✨ Try a Sample                            │   │
│  │  Listen to a 5-minute demo to see how      │   │
│  │  Disona transforms documents into audio    │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│           ─── or ───                                │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  + Upload your first PDF                    │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Definition of Done

- [ ] Sample appears for new users
- [ ] One-tap adds sample to library
- [ ] Audio plays immediately (pre-generated)
- [ ] All modes work (Blitz, Full, Go Deep)
- [ ] Sample marked clearly in UI
- [ ] Sample doesn't count against quota

---

## Dependencies

| Dependency | Status |
|------------|--------|
| Story 1.1 (OAuth) | Ready for Dev |
| Audio player (E4) | Backlog (can test basic) |
| Sample content | 🔴 Needs creation |

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 1.5 Guided First Upload | Follows sample experience |
| 6.7 Empty Library State | Uses same component |
