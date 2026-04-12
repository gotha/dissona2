# Library Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

The Library is the home screen after login. It displays the user's projects organized by recency, with filtering and status indicators. This is where users manage their content and initiate playback.

---

## Screen Structure

```
┌─────────────────────────────────────────┐
│  Disona                        [Avatar] │
├─────────────────────────────────────────┤
│                                         │
│  [All] [In Progress] [Completed]        │
│                                         │
│  ─────────────────────────────────────  │
│                                         │
│  Continue Listening                     │
│  ┌─────────────────────────────────┐   │
│  │ 📘 Deep Work          Ch 4  45% │   │
│  └─────────────────────────────────┘   │
│                                         │
│  Recent Projects                        │
│  ┌─────────────────────────────────┐   │
│  │ 📕 Atomic Habits      Ch 2  12% │   │
│  │ 📗 The Mom Test       ✓ Done    │   │
│  │ 📙 Zero to One        Ch 1   0% │   │
│  └─────────────────────────────────┘   │
│                                         │
│                          ┌───────────┐  │
│                          │ + Upload  │  │
│                          └───────────┘  │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │ advancement ▶ Deep Work Ch4    │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

**Key Elements:**
- Header with app name + avatar (account access)
- Filter tabs: All, In Progress, Completed
- Continue Listening section (most recent active)
- Recent Projects list
- Upload FAB (floating action button)
- Mini player (if audio playing)

---

## Filter Tabs

| Tab | Shows |
|-----|-------|
| **All** | All projects (default) |
| **In Progress** | Projects with 0% < progress < 100% |
| **Completed** | Projects with 100% progress |

**Sorting:** Most recently played first (within each filter)

---

## Project Card States

### Ready (Not Started)

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                            │
│    12 chapters • 4h 30m                 │
│    ────────────────────────── 0%       │
└─────────────────────────────────────────┘
```

### In Progress

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                            │
│    Chapter 4 of 12 • 2h 15m left        │
│    ████████████░░░░░░░░░░░░░░ 45%      │
└─────────────────────────────────────────┘
```

### Completed

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                     ✓      │
│    Completed • 4h 30m                   │
│    ██████████████████████████ 100%     │
└─────────────────────────────────────────┘
```

### Processing

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                            │
│    ⏳ Analyzing content...              │
│    ░░░░░░░░░░░░░░░░░░░░░░░░░           │
└─────────────────────────────────────────┘
```

### Audio Generation Pending

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                            │
│    12 chapters • Ready to generate      │
│    🎧 Generate Audio                    │
└─────────────────────────────────────────┘
```

### Audio Generating

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                            │
│    🔊 Generating audio... Ch 3/12       │
│    ████████░░░░░░░░░░░░░░░░░░ 25%      │
└─────────────────────────────────────────┘
```

### Failed

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                     ⚠️     │
│    Processing failed                    │
│    [View Details]                       │
└─────────────────────────────────────────┘
```

---

## Project Card Information

| Field | Source | Display |
|-------|--------|---------|
| **Title** | PDF metadata or filename | "Deep Work" |
| **Cover** | Generated from title | 📘 colored icon |
| **Chapter** | Current playback position | "Chapter 4 of 12" |
| **Time Left** | Calculated from position | "2h 15m left" |
| **Progress** | Percentage bar | Visual + "45%" |
| **Status** | Processing state | Icon + text |

---

## Continue Listening Section

Shows **only** the most recently played project that is not completed.

**Rules:**
- Only appears if there's an in-progress project
- Single project, not a list
- Larger card format for emphasis
- Tap → Resume playback immediately

```
┌─────────────────────────────────────────┐
│  Continue Listening                     │
│  ┌─────────────────────────────────┐   │
│  │                                  │   │
│  │  📘 Deep Work                   │   │
│  │                                  │   │
│  │  Chapter 4: Deep Work Is Rare   │   │
│  │  ████████████░░░░░░ 45%  ▶      │   │
│  │                                  │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

---

## Empty States

### No Projects (First Visit)

```
┌─────────────────────────────────────────┐
│  Disona                        [Avatar] │
├─────────────────────────────────────────┤
│                                         │
│                                         │
│              📚                         │
│                                         │
│      Your library is empty              │
│                                         │
│      Upload a PDF to transform it       │
│      into audio you can listen to       │
│      anywhere.                          │
│                                         │
│         ┌─────────────────┐             │
│         │  + Upload a file │             │
│         └─────────────────┘             │
│                                         │
│                                         │
└─────────────────────────────────────────┘
```

### Filter Returns No Results

```
┌─────────────────────────────────────────┐
│  [All] [In Progress] [Completed]        │
│                 ▲ selected              │
│  ─────────────────────────────────────  │
│                                         │
│              📭                         │
│                                         │
│      No completed projects yet          │
│                                         │
│      Keep listening to finish           │
│      your first book!                   │
│                                         │
└─────────────────────────────────────────┘
```

---

## Project Actions

### Tap on Project Card

| State | Action |
|-------|--------|
| Processing | Open project detail (show progress) |
| Ready (no audio) | Open project detail |
| Ready (has audio) | **Resume playback** |
| Completed | Open project detail |
| Failed | Open project detail (show error) |

### Long Press / Context Menu

```
┌─────────────────────────────┐
│  📘 Deep Work               │
├─────────────────────────────┤
│  ▶  Play                    │
│  📥 Download                │
│  📤 Share                   │
│  ✏️  Rename                 │
│  🗑️  Delete                 │
└─────────────────────────────┘
```

**Actions:**
- **Play:** Resume or start playback
- **Download:** Download for offline
- **Share:** Open share flow
- **Rename:** Edit project title
- **Delete:** Confirm + delete project

---

## Pull to Refresh

Pull down from top of library list to:
- Refresh project statuses
- Sync playback progress from server
- Check for completed audio generation

```
┌─────────────────────────────────────────┐
│         ↓ Pull to refresh               │
│  ─────────────────────────────────────  │
│                                         │
│  Recent Projects                        │
│  ...                                    │
└─────────────────────────────────────────┘
```

---

## Avatar / Account Menu

Tap avatar in header to open account menu:

```
┌─────────────────────────────────────────┐
│  ┌──────────────────────────────────┐  │
│  │  👤 gotha@example.com            │  │
│  │     Standard Plan                │  │
│  ├──────────────────────────────────┤  │
│  │  📊 Usage                        │  │
│  │     Upload: 12h / 50h            │  │
│  │     Audio:  8h / 20h             │  │
│  ├──────────────────────────────────┤  │
│  │  ⚙️  Settings                    │  │
│  │  💳 Subscription                 │  │
│  │  🚪 Sign Out                     │  │
│  └──────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

---

## Mini Player

When audio is playing, show persistent mini player at bottom:

```
┌─────────────────────────────────────────┐
│  📘 Deep Work • Ch 4       ▶  ▷▷       │
│  ████████████░░░░░░░░░░░░░░░  12:34    │
└─────────────────────────────────────────┘
```

**Interactions:**
- Tap → Expand to full player
- Tap play/pause → Toggle playback
- Tap skip → Next segment
- Swipe up → Expand to full player

---

## Shared With Me Section

If user has received shared content:

```
┌─────────────────────────────────────────┐
│  Shared With Me                    (3)  │
│  ┌─────────────────────────────────┐   │
│  │ 📕 Thinking Fast    from: John  │   │
│  │ 📗 Sapiens          from: Sarah │   │
│  └─────────────────────────────────┘   │
│  [See All →]                            │
└─────────────────────────────────────────┘
```

**Rules:**
- Shows max 2-3 items inline
- "See All" opens dedicated shared content view
- Hidden if no shared content

---

## Data Loading

### Initial Load

```
┌─────────────────────────────────────────┐
│  Disona                        [Avatar] │
├─────────────────────────────────────────┤
│                                         │
│        ◌ Loading your library...        │
│                                         │
└─────────────────────────────────────────┘
```

### Skeleton Loading

```
┌─────────────────────────────────────────┐
│  Recent Projects                        │
│  ┌─────────────────────────────────┐   │
│  │ ░░░░░░░░░░░░░░                  │   │
│  │ ░░░░░░░░░░░░░░░░░░              │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │ ░░░░░░░░░░░░                    │   │
│  │ ░░░░░░░░░░░░░░░░                │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

### Error State

```
┌─────────────────────────────────────────┐
│                                         │
│        ⚠️ Couldn't load library         │
│                                         │
│        Check your connection            │
│                                         │
│           [Try Again]                   │
│                                         │
└─────────────────────────────────────────┘
```

---

## Offline Behavior

| Scenario | Behavior |
|----------|----------|
| Fully offline | Show cached library, badge downloaded items |
| Downloaded content | Plays normally |
| Non-downloaded | Show "Offline" badge, tap shows download prompt |
| Sync pending | Show sync indicator, queue changes |

### Downloaded Badge

```
┌─────────────────────────────────────────┐
│ 📘 Deep Work                    📥      │
│    Chapter 4 of 12 • Downloaded         │
└─────────────────────────────────────────┘
```

---

## API Interactions

### Fetch Library

```http
GET /api/projects?sort=last_played&limit=50
```

**Response:**
```json
{
  "projects": [
    {
      "id": "uuid",
      "title": "Deep Work",
      "status": "ready",
      "audiobook_status": "ready",
      "chapters_total": 12,
      "progress": {
        "current_chapter": 4,
        "position_ms": 123456,
        "percentage": 45,
        "listening_mode": "blitz"
      },
      "estimated_duration_min": 270,
      "last_played_at": "2026-04-12T10:30:00Z",
      "is_downloaded": true
    }
  ],
  "shared_with_me": [
    {
      "id": "uuid",
      "title": "Thinking Fast and Slow",
      "shared_by": "john@example.com",
      "shared_at": "2026-04-10T15:00:00Z"
    }
  ]
}
```

### Delete Project

```http
DELETE /api/projects/{id}
```

**Confirmation required in UI before calling.**

---

## Navigation

| From Library | To |
|--------------|-----|
| Tap project (has audio) | Full Player |
| Tap project (no audio) | Project Detail |
| Tap "Upload a file" | Upload Flow |
| Tap Avatar | Account Menu |
| Tap mini player | Full Player |
| Tap "Shared With Me" | Shared Content View |

---

## Responsive Behavior

### Mobile (< 640px)
- Single column layout
- FAB in bottom-right corner
- Full-width project cards

### Tablet (640px - 1024px)
- Two column grid for projects
- Larger project cards
- FAB remains in corner

### Desktop (> 1024px)
- Three column grid
- Sidebar navigation option
- Upload button in header (not FAB)

---

## Accessibility

| Element | A11y |
|---------|------|
| Project cards | `role="listitem"`, focusable |
| Filter tabs | `role="tablist"` with `aria-selected` |
| Progress bars | `role="progressbar"` |
| Mini player | `role="region"` with `aria-label="Audio player"` |
| Empty states | Descriptive text for screen readers |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Library load time | <500ms (cached) |
| Tap to playback | <300ms |
| Filter response | <100ms |
| Refresh completion | <2s |

---

## Open Questions

1. **Search:** Add search bar in library for MVP?
2. **Folders:** Defer folder organization to post-MVP?
3. **Bulk actions:** Select multiple projects for download/delete?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
