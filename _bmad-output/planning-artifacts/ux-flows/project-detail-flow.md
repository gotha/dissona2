# Project Detail Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

The Project Detail view is the central hub for a single project. It shows the project's structure (chapters), processing status, audio generation controls, and playback options. Users arrive here after upload completes or by tapping a project from the Library.

---

## Entry Points

| From | Trigger | State |
|------|---------|-------|
| Upload flow | Processing complete | Fresh, no audio |
| Library | Tap project card | Any state |
| Player | Tap chapter browser | Playing |
| Share link | Recipient opens | Shared content |

---

## Screen Structure

```
┌─────────────────────────────────────────────────────┐
│  ←  Deep Work                            ⋮         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │                                             │   │
│  │              📘                             │   │
│  │                                             │   │
│  │         Deep Work                           │   │
│  │         Cal Newport                         │   │
│  │                                             │   │
│  │    12 chapters • 4h 30m estimated           │   │
│  │                                             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           🎧 Generate Audio                 │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Chapters                                           │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  1. Introduction                     15 min │   │
│  │  2. Deep Work Is Valuable            28 min │   │
│  │  3. Deep Work Is Rare                22 min │   │
│  │  4. Deep Work Is Meaningful          25 min │   │
│  │  5. Rule #1: Work Deeply             35 min │   │
│  │  ...                                        │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Header Section

### Project Info Card

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                    ┌─────────┐                      │
│                    │   📘    │  ← cover (100x100)   │
│                    └─────────┘                      │
│                                                     │
│              Deep Work                              │  ← title
│              Cal Newport                            │  ← author (if detected)
│                                                     │
│         12 chapters • 4h 30m estimated              │  ← stats
│                                                     │
│         📄 deep-work.pdf (2.4 MB)                   │  ← source file
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Stats Display

| Stat | Source | Example |
|------|--------|---------|
| Chapters | Count from parsing | "12 chapters" |
| Duration | Sum of estimated times | "4h 30m estimated" |
| Source | Original filename | "deep-work.pdf" |
| Author | PDF metadata | "Cal Newport" |

---

## Project States

### State 1: Processing

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│                                                     │
│         ⏳ Processing your document...              │
│                                                     │
│         Detecting chapters...                       │
│         ████████████░░░░░░░░░░░░░░░  45%           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### State 2: Ready (No Audio)

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│         12 chapters • 4h 30m estimated              │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           🎧 Generate Audio                 │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  Chapters                                           │
│  1. Introduction                           15 min   │
│  2. Deep Work Is Valuable                  28 min   │
│  ...                                               │
└─────────────────────────────────────────────────────┘
```

### State 3: Generating Audio

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│         12 chapters • 4h 30m estimated              │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  🔊 Generating audio...                     │   │
│  │  Chapter 3 of 12                            │   │
│  │  ████████████░░░░░░░░░░░░░░░  25%          │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  Chapters                                           │
│  1. Introduction                    ✓ Ready        │
│  2. Deep Work Is Valuable           ✓ Ready        │
│  3. Deep Work Is Rare               ⏳ Generating   │
│  4. Deep Work Is Meaningful         ○ Queued       │
│  ...                                               │
└─────────────────────────────────────────────────────┘
```

### State 4: Audio Ready

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│         12 chapters • 4h 30m                        │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           ▶ Play                            │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [⚡ Blitz]  [📖 Full]        📥 Download  📤 Share │
│                                                     │
│  Chapters                                           │
│  1. Introduction                    ✓ 15 min       │
│  2. Deep Work Is Valuable           ✓ 28 min       │
│  ...                                               │
└─────────────────────────────────────────────────────┘
```

### State 5: In Progress (Has Playback)

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│         Chapter 4 of 12 • 2h 15m left               │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           ▶ Continue                        │   │
│  │     Ch 4: Deep Work Is Meaningful @ 12:34   │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [⚡ Blitz]  [📖 Full]        📥 Download  📤 Share │
│                                                     │
│  Chapters                                           │
│  1. Introduction                    ✓ Complete     │
│  2. Deep Work Is Valuable           ✓ Complete     │
│  3. Deep Work Is Rare               ✓ Complete     │
│  4. Deep Work Is Meaningful         ▶ 45%          │
│  5. Rule #1: Work Deeply            ○ 35 min       │
│  ...                                               │
└─────────────────────────────────────────────────────┘
```

### State 6: Failed

```
┌─────────────────────────────────────────────────────┐
│              Deep Work                              │
│                                                     │
│         ⚠️ Processing failed                        │
│                                                     │
│         Could not extract text from PDF.            │
│         The file may be image-only or corrupted.    │
│                                                     │
│         ┌─────────────────┐  ┌─────────────────┐   │
│         │     Retry       │  │     Delete      │   │
│         └─────────────────┘  └─────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Generate Audio Flow

### Generate Audio Button

```
┌─────────────────────────────────────────────────────┐
│  ┌─────────────────────────────────────────────┐   │
│  │           🎧 Generate Audio                 │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│         Uses 4h 30m of your audio quota            │
│         (12h remaining)                             │
└─────────────────────────────────────────────────────┘
```

### Generation Options Modal

```
┌─────────────────────────────────────────────────────┐
│  Generate Audio                              ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  What would you like to generate?                   │
│                                                     │
│  ○ All chapters (12)                    4h 30m     │
│  ○ Selected chapters                    —          │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Voice: Sarah (Default)                    [Change] │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Quota: 12h remaining                               │
│  This will use: 4h 30m                              │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Generate                          │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Select Chapters (If Selected)

```
┌─────────────────────────────────────────────────────┐
│  Select Chapters                             ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ☑ 1. Introduction                         15 min  │
│  ☑ 2. Deep Work Is Valuable                28 min  │
│  ☐ 3. Deep Work Is Rare                    22 min  │
│  ☑ 4. Deep Work Is Meaningful              25 min  │
│  ☐ 5. Rule #1: Work Deeply                 35 min  │
│  ☐ 6. Rule #2: Embrace Boredom             30 min  │
│  ...                                               │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Selected: 3 chapters • 1h 8m                       │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Generate Selected                 │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Quota Warning

```
┌─────────────────────────────────────────────────────┐
│  ⚠️ Quota Warning                            ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  This will use more than your remaining quota.      │
│                                                     │
│  Requested: 4h 30m                                  │
│  Remaining: 2h 15m                                  │
│  Overage: 2h 15m                                    │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Upgrade Plan                      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      Generate First 2h 15m Only             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Chapter List

### Chapter Item States

| State | Display |
|-------|---------|
| Not generated | `○ Chapter Title` + duration |
| Queued | `◔ Chapter Title` + "Queued" |
| Generating | `⏳ Chapter Title` + progress % |
| Ready (not played) | `✓ Chapter Title` + duration |
| In progress | `▶ Chapter Title` + progress % |
| Completed | `✓ Chapter Title` + "Complete" |
| Failed | `⚠️ Chapter Title` + "Failed" |

### Chapter Card (Ready, Not Played)

```
┌─────────────────────────────────────────────────────┐
│  ✓  4. Deep Work Is Meaningful              25 min │
│      4 key points                                   │
└─────────────────────────────────────────────────────┘
```

### Chapter Card (In Progress)

```
┌─────────────────────────────────────────────────────┐
│  ▶  4. Deep Work Is Meaningful              ▶ 45%  │
│      4 key points • 12:34 / 25:00                   │
│      ████████████░░░░░░░░░░░░░░░                   │
└─────────────────────────────────────────────────────┘
```

### Chapter Card (Expanded)

Tap chapter to expand:

```
┌─────────────────────────────────────────────────────┐
│  ✓  4. Deep Work Is Meaningful              25 min │
│                                                     │
│      Key Points:                                    │
│      • The Attention Residue Problem               │
│      • Why Depth Creates Meaning                   │
│      • The Craftsman Mindset                       │
│      • Neurological vs Psychological               │
│                                                     │
│      ┌──────────────┐  ┌──────────────┐           │
│      │  ⚡ Blitz    │  │  📖 Full     │           │
│      └──────────────┘  └──────────────┘           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Actions Bar

### When Audio Ready

```
┌─────────────────────────────────────────────────────┐
│  [⚡ Blitz]  [📖 Full]        📥 Download  📤 Share │
└─────────────────────────────────────────────────────┘
```

| Action | Behavior |
|--------|----------|
| ⚡ Blitz | Start playback in Blitz mode |
| 📖 Full | Start playback in Full mode |
| 📥 Download | Download all audio for offline |
| 📤 Share | Open share flow |

### Download States

```
Not downloaded:  📥 Download (240 MB)
Downloading:     📥 45% ████░░░░░░
Downloaded:      ✓ Downloaded
```

---

## Menu (⋮) Options

```
┌─────────────────────────────┐
│  📘 Deep Work               │
├─────────────────────────────┤
│  ✏️  Rename                 │
│  🎤 Change Voice            │
│  📥 Download All            │
│  📤 Share                   │
│  ───────────────────────    │
│  🗑️  Delete Project         │
└─────────────────────────────┘
```

### Delete Confirmation

```
┌─────────────────────────────────────────────────────┐
│  Delete Project?                             ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  This will permanently delete "Deep Work"           │
│  and all generated audio.                           │
│                                                     │
│  ⚠️ This cannot be undone.                          │
│                                                     │
│  ⚠️ 2 people have this shared. They will            │
│     lose access.                                    │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Delete                            │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│                    Cancel                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Voice Selection

### Change Voice Modal

```
┌─────────────────────────────────────────────────────┐
│  Select Voice                                ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Current: Sarah (American English)                  │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ○ Sarah        American English, Female    ▶ Play │
│  ○ Michael      American English, Male      ▶ Play │
│  ○ Emma         British English, Female     ▶ Play │
│  ○ James        British English, Male       ▶ Play │
│  ○ Sofia        Spanish, Female             ▶ Play │
│  ...                                               │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ⚠️ Changing voice requires regenerating audio.    │
│     This will use quota again.                      │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Apply & Regenerate                │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Tap Behaviors

| Element | Tap Action |
|---------|------------|
| Back arrow (←) | Return to Library |
| Menu (⋮) | Open options menu |
| Play/Continue button | Start/resume playback |
| Mode toggle (Blitz/Full) | Set mode, then play |
| Chapter (audio ready) | Play that chapter |
| Chapter (no audio) | Expand to show details |
| Download | Start download |
| Share | Open share flow |

---

## Playback Resume Logic

| Scenario | Button Text | Behavior |
|----------|-------------|----------|
| Never played | "▶ Play" | Start Chapter 1 |
| In progress | "▶ Continue" | Resume exact position |
| Completed | "▶ Play Again" | Restart from Chapter 1 |

---

## Multi-Document Projects

If project has multiple source documents:

```
┌─────────────────────────────────────────────────────┐
│              Leadership Bundle                      │
│         3 documents • 28 chapters • 12h            │
│                                                     │
│  Documents                                          │
│  ┌─────────────────────────────────────────────┐   │
│  │  📄 Deep Work.pdf                   12 ch   │   │
│  │  📄 Atomic Habits.pdf               10 ch   │   │
│  │  📄 The Mom Test.pdf                 6 ch   │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  All Chapters                                       │
│  ┌─────────────────────────────────────────────┐   │
│  │  Deep Work                                  │   │
│  │  1. Introduction                    15 min  │   │
│  │  2. Deep Work Is Valuable           28 min  │   │
│  │  ...                                        │   │
│  │                                             │   │
│  │  Atomic Habits                              │   │
│  │  1. The Surprising Power            20 min  │   │
│  │  ...                                        │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## API Interactions

### Fetch Project Detail

```http
GET /api/projects/{id}
```

**Response:**
```json
{
  "id": "uuid",
  "title": "Deep Work",
  "author": "Cal Newport",
  "status": "ready",
  "audiobook_status": "ready",
  "chapters_total": 12,
  "estimated_duration_min": 270,
  "actual_duration_min": 265,
  "documents": [
    {
      "id": "uuid",
      "filename": "deep-work.pdf",
      "file_size_bytes": 2400000,
      "chapters_count": 12
    }
  ],
  "chapters": [
    {
      "id": "uuid",
      "number": 1,
      "title": "Introduction",
      "duration_min": 15,
      "key_points_count": 3,
      "audio_status": "ready",
      "playback_progress": 100
    },
    {
      "id": "uuid",
      "number": 2,
      "title": "Deep Work Is Valuable",
      "duration_min": 28,
      "key_points_count": 4,
      "audio_status": "ready",
      "playback_progress": 0
    }
  ],
  "progress": {
    "current_chapter": 4,
    "position_ms": 754000,
    "percentage": 45,
    "listening_mode": "blitz"
  },
  "voice": {
    "id": "uuid",
    "name": "Sarah",
    "language": "en-US"
  },
  "download_status": "not_downloaded",
  "download_size_bytes": 251658240
}
```

### Trigger Audio Generation

```http
POST /api/projects/{id}/generate
Content-Type: application/json

{
  "chapters": ["all"],  // or ["uuid1", "uuid2"]
  "voice_id": "uuid"
}
```

### Delete Project

```http
DELETE /api/projects/{id}
```

---

## Polling During Generation

While `audiobook_status === "generating"`:

```javascript
const POLL_INTERVAL = 3000; // 3 seconds

async function pollGeneration(projectId) {
  const project = await api.getProject(projectId);

  updateChapterStatuses(project.chapters);
  updateOverallProgress(project);

  if (project.audiobook_status === 'ready') {
    showReadyState();
    return;
  }

  if (project.audiobook_status === 'failed') {
    showError(project);
    return;
  }

  setTimeout(() => pollGeneration(projectId), POLL_INTERVAL);
}
```

---

## Accessibility

| Element | A11y |
|---------|------|
| Chapter list | `role="list"` with `aria-label="Chapters"` |
| Chapter items | `role="listitem"` |
| Progress indicators | `aria-valuenow`, `aria-valuemax` |
| Generate button | `aria-describedby` with quota info |
| Mode toggle | `role="radiogroup"` |
| Download button | Announces state changes |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Generate conversion | >70% of processed projects |
| First play latency | <2s after tap |
| Chapter tap-to-play | <500ms |
| Download completion | >90% |

---

## Open Questions

1. **Chapter preview:** Show text snippet on expand?
2. **Batch operations:** Select multiple chapters for actions?
3. **Reading order:** Allow reordering chapters?
4. **Bookmarks:** Add bookmark feature to chapters?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
