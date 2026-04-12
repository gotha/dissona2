# Upload Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

The upload flow enables users to create projects by uploading PDF files. It uses a simple, single-file-first approach with optional multi-document support.

---

## Entry Point: Library Screen

### Default State (Has Projects)

```
┌─────────────────────────────────────────┐
│  Disona                        [Avatar] │
├─────────────────────────────────────────┤
│                                         │
│  Recently Played                        │
│  ┌─────────────────────────────────┐   │
│  │ 📘 Deep Work          ▶ 45% ── │   │
│  │ 📕 Atomic Habits      ▶ 12% ── │   │
│  │ 📗 The Mom Test       ✓ Done   │   │
│  └─────────────────────────────────┘   │
│                                         │
│                          ┌───────────┐  │
│                          │ + Upload  │  │
│                          │   a file  │  │
│                          └───────────┘  │
│                                         │
└─────────────────────────────────────────┘
```

### Empty State (No Projects)

```
┌─────────────────────────────────────────┐
│  Disona                        [Avatar] │
├─────────────────────────────────────────┤
│                                         │
│                                         │
│           📚                            │
│                                         │
│     Your library is empty               │
│                                         │
│     Upload a PDF to get started         │
│                                         │
│         ┌─────────────────┐             │
│         │  + Upload a file │             │
│         └─────────────────┘             │
│                                         │
│                                         │
└─────────────────────────────────────────┘
```

**Button:** "Upload a file" (may evolve to "Create a project" later)

---

## Step 1: File Selection

User taps "Upload a file" → Native file picker opens

**Accepted formats:** `.pdf` only (MVP)

**File size limit:** 100 MB (configurable)

---

## Step 2: Uploading State

Once file is selected, show upload progress:

```
┌─────────────────────────────────────────┐
│  Uploading...                           │
│                                         │
│  📄 deep-work.pdf                       │
│  ████████████░░░░░░░░░░  67%           │
│                                         │
│                          [Cancel]       │
└─────────────────────────────────────────┘
```

**Behavior:**
- Modal overlay or inline in library
- Shows filename + progress percentage
- Cancel button aborts upload
- On completion → transitions to Processing state

---

## Step 3: Processing State (In Library)

Project appears in library immediately with status:

```
┌─────────────────────────────────────────┐
│  Recently Played                        │
│  ┌─────────────────────────────────┐   │
│  │ 📘 Deep Work                    │   │
│  │    ⏳ Detecting chapters...     │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │ 📕 Atomic Habits      ▶ 12%    │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

### Processing Status Progression

| Status | Display Text |
|--------|--------------|
| `uploading` | "Uploading..." |
| `parsing` | "Parsing PDF..." |
| `detecting_chapters` | "Detecting chapters..." |
| `analyzing` | "Analyzing content..." |
| `generating_summaries` | "Generating summaries..." |
| `ready` | (no status, shows chapter count) |
| `failed` | "⚠️ Processing failed" |

**Polling:** Check status every 3 seconds until `ready` or `failed`

---

## Step 4: Completion → Auto-Open Project

When status becomes `ready`:
- Automatically navigate to Project View
- Show chapters with structure

```
┌─────────────────────────────────────────┐
│  ← Deep Work                   [•••]   │
├─────────────────────────────────────────┤
│                                         │
│  📄 deep-work.pdf                       │
│  12 chapters • 4h 30m estimated         │
│                                         │
│  ─────────────────────────────────────  │
│                                         │
│  Chapters                               │
│                                         │
│  1. Introduction                 15 min │
│  2. Deep Work Is Valuable        28 min │
│  3. Deep Work Is Rare            22 min │
│  4. Deep Work Is Meaningful      25 min │
│  ...                                    │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │      🎧 Generate Audio          │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

**State:** Project is now "locked" — no more files can be added.

---

## Multi-Document Support

### Adding Files (Before Processing Complete)

After first file uploads, user can add more:

```
┌─────────────────────────────────────────┐
│  ← New Project                  [•••]  │
├─────────────────────────────────────────┤
│                                         │
│  Documents (1)                          │
│  ┌─────────────────────────────────┐   │
│  │ 📄 deep-work.pdf                │   │
│  │    ⏳ Detecting chapters...     │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  + Add another file             │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

### When Can Files Be Added?

| Project State | Can Add Files? |
|---------------|----------------|
| Uploading | ✅ Yes |
| Processing | ✅ Yes |
| Failed | ✅ Yes (retry) |
| Ready (success) | ❌ No |

**Rationale:** Once chapters are generated, the structure is locked.

---

## Error States

### Upload Failed

```
┌─────────────────────────────────────────┐
│  ← New Project                  [•••]  │
├─────────────────────────────────────────┤
│                                         │
│  Documents (1)                          │
│  ┌─────────────────────────────────┐   │
│  │ 📄 deep-work.pdf                │   │
│  │    ⚠️ Upload failed             │   │
│  │                                  │   │
│  │    [Retry]  [Remove]            │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  + Add another file             │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

### Parsing Failed

```
┌─────────────────────────────────────────┐
│  Documents (1)                          │
│  ┌─────────────────────────────────┐   │
│  │ 📄 encrypted-doc.pdf            │   │
│  │    ⚠️ Processing failed         │   │
│  │    "Could not read PDF"         │   │
│  │                                  │   │
│  │    [Retry]  [Remove]            │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  + Add another file             │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

**Error messages:**
- "Could not read PDF" — encrypted or corrupted
- "PDF too large" — exceeds size limit
- "No text found" — image-only PDF without OCR success
- "Processing error" — generic fallback

---

## State Machine

```
                    ┌─────────┐
                    │  START  │
                    └────┬────┘
                         │
                         ▼
                   ┌───────────┐
                   │ Uploading │──────────┐
                   └─────┬─────┘          │
                         │                │ (error)
                         ▼                ▼
                   ┌───────────┐    ┌──────────┐
                   │ Processing │    │  Failed  │◄────┐
                   └─────┬─────┘    └────┬─────┘     │
                         │               │           │
                         │               └───────────┤
                         ▼                           │
                   ┌───────────┐                     │
                   │   Ready   │                     │
                   │  (locked) │                     │
                   └───────────┘                     │
                                                     │
              User can add files ─────────────────────┘
              (retry workflow)
```

---

## API Interactions

### 1. Create Project + Upload

```http
POST /api/projects
Content-Type: multipart/form-data

file: <PDF binary>
title: "Deep Work" (optional, defaults to filename)
```

**Response:**
```json
{
  "id": "uuid",
  "title": "Deep Work",
  "status": "uploading",
  "documents": [{
    "id": "uuid",
    "filename": "deep-work.pdf",
    "status": "uploading"
  }]
}
```

### 2. Poll Status

```http
GET /api/projects/{id}
```

**Response (processing):**
```json
{
  "id": "uuid",
  "title": "Deep Work",
  "status": "processing",
  "substatus": "detecting_chapters",
  "documents": [{
    "id": "uuid",
    "filename": "deep-work.pdf",
    "status": "processing"
  }]
}
```

**Response (ready):**
```json
{
  "id": "uuid",
  "title": "Deep Work",
  "status": "ready",
  "chapters_total": 12,
  "estimated_duration_min": 270,
  "documents": [{
    "id": "uuid",
    "filename": "deep-work.pdf",
    "status": "processed",
    "chapters_count": 12
  }],
  "chapters": [
    {"number": 1, "title": "Introduction", "estimated_min": 15},
    ...
  ]
}
```

### 3. Add Document to Project

```http
POST /api/projects/{id}/documents
Content-Type: multipart/form-data

file: <PDF binary>
```

**Only allowed when:** `project.status != "ready"`

---

## Polling Strategy

```typescript
const POLL_INTERVAL = 3000; // 3 seconds
const MAX_POLLS = 200;      // 10 minutes max

async function pollProjectStatus(projectId: string) {
  let polls = 0;

  while (polls < MAX_POLLS) {
    const project = await api.getProject(projectId);

    updateUI(project);

    if (project.status === 'ready') {
      navigateToProjectView(project);
      return;
    }

    if (project.status === 'failed') {
      showError(project);
      return;
    }

    await sleep(POLL_INTERVAL);
    polls++;
  }

  showTimeoutError();
}
```

---

## Design Tokens

| Element | Style |
|---------|-------|
| Upload button | Primary (green), full-width on empty state |
| Progress bar | Green fill, gray background |
| Status text | Secondary color, small text |
| Error state | Red accent, warning icon |
| Add file button | Outline style, secondary |

---

## Mobile Considerations

### Touch Targets
- Upload button: minimum 48x48px tap area
- File items: full-width tap to expand details

### File Picker
- Uses native `<input type="file" accept=".pdf">`
- Falls back gracefully on all mobile browsers

### Progress Feedback
- Haptic feedback on upload complete
- Push notification if app backgrounded during processing

---

## Accessibility

| Element | A11y |
|---------|------|
| Upload button | `aria-label="Upload a PDF file"` |
| Progress bar | `role="progressbar"` with `aria-valuenow` |
| Status updates | `aria-live="polite"` for screen readers |
| Error messages | `role="alert"` |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Upload completion rate | >95% |
| Processing success rate | >90% |
| Time to first chapter visible | <30 sec |
| Error recovery rate | >50% retry |

---

## Open Questions

1. **Project naming:** Auto-extract from PDF metadata or filename?
2. **Large files:** Show estimated processing time?
3. **Background upload:** Continue if user navigates away?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
