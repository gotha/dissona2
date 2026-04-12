# Story 1.5: Guided First Upload

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 3  
**Created:** 2026-04-12

---

## User Story

As a **new user**,  
I want to **be guided through my first upload**,  
So that I can **successfully create my first audiobook**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR57 | Guided onboarding for first upload |

---

## Acceptance Criteria

### AC1: First Upload Prompt

- **Given** I am a new user with no projects (or only sample)
- **When** I tap "Upload your first PDF"
- **Then** I see a guided upload flow with helpful tips
- **And** tips explain what to expect

### AC2: Step-by-Step Guidance

- **Given** I am in the guided upload flow
- **When** I progress through steps
- **Then** I see:
  - Step 1: File selection tips
  - Step 2: Upload progress with reassurance
  - Step 3: Processing explanation
  - Step 4: Completion celebration

### AC3: Encouraging Processing Messages

- **Given** my file is processing
- **When** I see the processing screen
- **Then** I see encouraging messages like:
  - "Analyzing your document..."
  - "Found 12 chapters! 📚"
  - "Almost there..."

### AC4: Completion Celebration

- **Given** processing completes
- **When** I see the completion screen
- **Then** I see a celebration UI (confetti, success message)
- **And** I'm prompted: "Play Now" or "Generate Audio"

### AC5: Skip Option

- **Given** I am an experienced user
- **When** I see the guided flow
- **Then** I can tap "Skip tutorial" to use standard upload

### AC6: First Upload Flag

- **Given** I complete my first guided upload
- **When** I upload subsequent files
- **Then** I see the standard upload flow (not guided)

---

## Technical Design

### Guided Flow States

```
┌──────────────────────────────────────────────────────────────┐
│                      GUIDED UPLOAD FLOW                       │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  [Tips] ──► [Select File] ──► [Uploading] ──► [Processing]   │
│                                                   │           │
│                                                   ▼           │
│                                            [Celebration]      │
│                                                   │           │
│                                                   ▼           │
│                                          [Project Detail]     │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### Components to Create

| Component | File | Description |
|-----------|------|-------------|
| GuidedUpload | `frontend/src/components/onboarding/GuidedUpload.tsx` | Main flow |
| UploadTips | `frontend/src/components/onboarding/UploadTips.tsx` | Tip cards |
| ProcessingMessages | `frontend/src/components/onboarding/ProcessingMessages.tsx` | Encouraging text |
| Celebration | `frontend/src/components/onboarding/Celebration.tsx` | Success screen |

### User Flags

Add to user record:
```sql
ALTER TABLE users ADD COLUMN has_completed_first_upload BOOLEAN DEFAULT FALSE;
```

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.5.1** Add `has_completed_first_upload` to users table
- [ ] **1.5.2** Update user flag on first upload completion
- [ ] **1.5.3** Return flag in user profile response

### Frontend Tasks

- [ ] **1.5.4** Create GuidedUpload component
- [ ] **1.5.5** Create UploadTips component
- [ ] **1.5.6** Create ProcessingMessages component with rotating text
- [ ] **1.5.7** Create Celebration component with confetti
- [ ] **1.5.8** Add guided flow routing logic
- [ ] **1.5.9** Add "Skip tutorial" option
- [ ] **1.5.10** Unit tests for guided components
- [ ] **1.5.11** E2E test for guided flow

---

## UI Wireframes

### Step 1: Tips

```
┌─────────────────────────────────────────────────────┐
│  Upload your first PDF                       ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📄 Tips for best results:                          │
│                                                     │
│  ✓ Text-based PDFs work best                       │
│  ✓ Chapters are auto-detected                      │
│  ✓ Processing takes 1-2 minutes                    │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      Select a PDF file                      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│                    Skip tutorial                    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Step 2: Processing with Encouragement

```
┌─────────────────────────────────────────────────────┐
│  Creating your audiobook...                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│                    📚                               │
│                                                     │
│         "Found 12 chapters!"                        │
│                                                     │
│  ████████████████░░░░░░░░░░░░  65%                │
│                                                     │
│  Analyzing content structure...                     │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  💡 Tip: Disona creates summaries so you can       │
│     get the key points in minutes!                  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Step 3: Celebration

```
┌─────────────────────────────────────────────────────┐
│                      🎉                             │
├─────────────────────────────────────────────────────┤
│                                                     │
│         Your audiobook is ready!                    │
│                                                     │
│              📘 Deep Work                           │
│           12 chapters • 4h 30m                      │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      🎧 Generate Audio                      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      📖 View Chapters                       │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Definition of Done

- [ ] New users see guided flow on first upload
- [ ] Tips display before file selection
- [ ] Encouraging messages show during processing
- [ ] Celebration displays on completion
- [ ] Skip option works
- [ ] Subsequent uploads use standard flow

---

## Dependencies

| Dependency | Status |
|------------|--------|
| Story 2.1 (Upload) | Backlog |
| Story 2.2 (Processing) | Backlog |

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 1.4 Sample Content | Precedes this flow |
| 2.1 PDF Upload | Wraps this flow |
| 2.4 Processing Status | Uses processing messages |
