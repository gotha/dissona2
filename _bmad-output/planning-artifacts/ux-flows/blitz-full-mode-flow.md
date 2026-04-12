# Blitz & Full Mode Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

Disona's core differentiator is **"Depth on Demand"** — users control how deep they go into content. Two listening modes enable this:

| Mode | What You Hear | Duration | Use Case |
|------|---------------|----------|----------|
| **Blitz ⚡** | Chapter summaries + key point summaries | ~15-20% of full | Quick overview, triage |
| **Full 📖** | Complete chapter narration | 100% | Deep learning |

Users can switch modes at any time and seamlessly "Go Deep" from a summary into the full content.

---

## Content Hierarchy

### Three-Level Structure

```
Project (Book)
└── Chapter 1
    ├── L1: Chapter Summary (~1 min)
    ├── L2: Key Point 1 Summary (~45 sec)
    ├── L2: Key Point 2 Summary (~45 sec)
    ├── L2: Key Point 3 Summary (~45 sec)
    └── L2: Key Point 4 Summary (~45 sec)
    
    Full Chapter: Complete narration (~25-40 min)
    └── Segment 1 (maps to KP1)
    └── Segment 2 (maps to KP2)
    └── Segment 3 (maps to KP3)
    └── Segment 4 (maps to KP4)
```

### Content Mapping

| Blitz Content | Maps To Full Content |
|---------------|---------------------|
| L1 Chapter Summary | Chapter start (0:00) |
| L2 Key Point 1 | Segment 1 start |
| L2 Key Point 2 | Segment 2 start |
| L2 Key Point 3 | Segment 3 start |
| L2 Key Point 4 | Segment 4 start |

---

## Blitz Mode ⚡

### Playback Flow

```
Chapter 1:
  L1 Summary (1 min)
    ↓ auto-advance
  L2 KP1 (45 sec) → [Go Deep?]
    ↓ auto-advance
  L2 KP2 (45 sec) → [Go Deep?]
    ↓ auto-advance
  L2 KP3 (45 sec) → [Go Deep?]
    ↓ auto-advance
  L2 KP4 (45 sec) → [Go Deep?]
    ↓ auto-advance
Chapter 2:
  L1 Summary (1 min)
    ↓ ...continues
```

### Blitz Player UI

```
┌─────────────────────────────────────────────────────┐
│  ↓                              ⋮                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│                    ┌─────────┐                      │
│                    │   📘    │                      │
│                    └─────────┘                      │
│                                                     │
│               Deep Work                             │
│     ⚡ Chapter 4: Deep Work Is Meaningful           │
│                                                     │
│  ┌───────────────────────────────────────────────┐ │
│  │  Key Point 2 of 4                             │ │
│  │  The Attention Residue Problem                │ │
│  └───────────────────────────────────────────────┘ │
│                                                     │
│   0:23  ████████████░░░░░░░░░░░░░░░░░░  0:45      │
│                                                     │
│       ⏮️        ◀◀        ▶️        ▶▶        ⏭️     │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│              ┌─────────────────────┐                │
│              │    🔍 Go Deep       │                │
│              └─────────────────────┘                │
│         Hear the full chapter from here             │
│                                                     │
│   [⚡ Blitz]  [📖 Full]           1.5x    📥  📤   │
│       ▲ active                                      │
└─────────────────────────────────────────────────────┘
```

### Blitz Navigation

| Button | Action |
|--------|--------|
| ⏮️ Prev | Previous L2 (or L1 if at first L2) |
| ⏭️ Next | Next L2 (or next chapter L1 if at last L2) |
| ◀◀ -15s | Rewind 15 seconds |
| ▶▶ +15s | Forward 15 seconds |

### Progress Indicator (Blitz)

Shows position within chapter's summaries:

```
Chapter 4: Deep Work Is Meaningful
  ● L1    ● KP1    ◉ KP2    ○ KP3    ○ KP4
                    ▲
                 current
```

---

## Full Mode 📖

### Playback Flow

```
Chapter 1:
  Full Narration (25-40 min)
    └── Segment 1 (KP1 region)
    └── Segment 2 (KP2 region)
    └── Segment 3 (KP3 region)
    └── Segment 4 (KP4 region)
    ↓ auto-advance
Chapter 2:
  Full Narration
    ↓ ...continues
```

### Full Mode Player UI

```
┌─────────────────────────────────────────────────────┐
│  ↓                              ⋮                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│                    ┌─────────┐                      │
│                    │   📘    │                      │
│                    └─────────┘                      │
│                                                     │
│               Deep Work                             │
│     📖 Chapter 4: Deep Work Is Meaningful           │
│                                                     │
│  ┌───────────────────────────────────────────────┐ │
│  │  Currently in: Key Point 2 region             │ │
│  │  The Attention Residue Problem                │ │
│  └───────────────────────────────────────────────┘ │
│                                                     │
│   12:34  ███████░░░░░░░░░░░░░░░░░░░░░░  32:00     │
│          │    │    │    │                          │
│         KP1  KP2  KP3  KP4  ← key point markers    │
│                                                     │
│       ⏮️        ◀◀        ▶️        ▶▶        ⏭️     │
│                                                     │
│   [⚡ Blitz]  [📖 Full]           1.5x    📥  📤   │
│                  ▲ active                           │
└─────────────────────────────────────────────────────┘
```

### Full Mode Navigation

| Button | Action |
|--------|--------|
| ⏮️ Prev | Previous segment/key point region |
| ⏭️ Next | Next segment/key point region |
| ◀◀ -15s | Rewind 15 seconds |
| ▶▶ +15s | Forward 15 seconds |

### Progress Bar with Key Point Markers

```
┌─────────────────────────────────────────────────────┐
│   0:00  ███████│███│███│░░░░│░░░░░░░░░░░░  32:00   │
│                ▲   ▲   ▲    ▲                       │
│               KP1 KP2 KP3  KP4                      │
│                    ▲                                │
│              current position                       │
└─────────────────────────────────────────────────────┘
```

**Tap on marker** → Jump to that key point's position

---

## Go Deep — The Core Transition

### When Go Deep Appears

- Only in **Blitz Mode**
- During L2 key point playback
- Not shown during L1 chapter summary

### Go Deep Button

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  ⚡ Key Point 2: The Attention Residue Problem      │
│                                                     │
│              ┌─────────────────────┐                │
│              │    🔍 Go Deep       │                │
│              └─────────────────────┘                │
│                                                     │
│         Hear the full chapter from here             │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Go Deep Behavior

**User taps "Go Deep" during KP2:**

1. **Transition** (200ms crossfade)
2. **Switch to Full Mode**
3. **Jump to KP2 position** in full chapter (e.g., 12:34)
4. **Continue playback** from that point

```
BEFORE (Blitz):          AFTER (Full):
┌──────────────────┐     ┌──────────────────┐
│ ⚡ KP2 Summary    │ ──► │ 📖 Full @ 12:34  │
│ 0:23 / 0:45      │     │ 12:34 / 32:00    │
└──────────────────┘     └──────────────────┘
```

### After Go Deep — Return to Blitz?

**Default behavior:** Temporary deep mode

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  After this chapter ends:                           │
│                                                     │
│  ○ Return to Blitz Mode (next chapter summary)     │ ← default
│  ○ Stay in Full Mode (continue full chapters)      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Checkbox: "Stay in Full Mode"**
- Unchecked (default): After chapter ends → Return to Blitz (next L1)
- Checked: After chapter ends → Stay in Full (next full chapter)

---

## Mode Switching

### Toggle Button

```
┌──────────────────────────────────────┐
│   [⚡ Blitz]    [📖 Full]            │
│       ▲                              │
│    selected                          │
└──────────────────────────────────────┘
```

### Blitz → Full Switch

**Position mapping:**

| Currently Playing | Jump To |
|-------------------|---------|
| L1 Chapter Summary | Full chapter at 0:00 |
| L2 Key Point 1 | Full chapter at KP1 timestamp |
| L2 Key Point 2 | Full chapter at KP2 timestamp |
| L2 Key Point N | Full chapter at KPN timestamp |

**Animation:** 150ms mode indicator switch + position update

### Full → Blitz Switch

**Position mapping:**

| Currently At | Jump To |
|--------------|---------|
| Before KP1 marker | L1 Chapter Summary |
| In KP1 region | L2 Key Point 1 |
| In KP2 region | L2 Key Point 2 |
| In KPN region | L2 Key Point N |

**Note:** Loses position within summary (starts from beginning of that L2)

---

## Chapter Completion

### End of Chapter in Blitz Mode

After last L2 key point:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│           ✓ Chapter 4 Complete (Blitz)              │
│                                                     │
│     Listened to summary + 4 key points              │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  ▶ Next Chapter     │                     │
│         └─────────────────────┘                     │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  🔍 Go Deep on Ch4  │                     │
│         └─────────────────────┘                     │
│                                                     │
│         Playing next in 5s...                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Options:**
- Auto-advance to next L1 (default, 5s timer)
- Tap "Go Deep" → Listen to full Ch4 now
- Tap "Next Chapter" → Skip to Ch5 L1 immediately

### End of Chapter in Full Mode

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│           ✓ Chapter 4 Complete (Full)               │
│                                                     │
│     32 minutes • Full narration                     │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  ▶ Next Chapter     │                     │
│         └─────────────────────┘                     │
│                                                     │
│         Playing next in 5s...                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**If "Return to Blitz" was set:** Next chapter plays in Blitz (L1 summary)
**If "Stay in Full" was set:** Next chapter plays in Full

---

## Visual Differentiation

### Blitz Mode Indicators

| Element | Style |
|---------|-------|
| Mode badge | ⚡ Amber/yellow accent |
| Progress bar | Amber color |
| Chapter label | "⚡ Chapter 4" prefix |
| Key point card | Highlighted container |

### Full Mode Indicators

| Element | Style |
|---------|-------|
| Mode badge | 📖 Blue accent |
| Progress bar | Blue color |
| Chapter label | "📖 Chapter 4" prefix |
| Key point markers | Subtle tick marks on progress |

### Color Tokens

```
--blitz-accent: #F59E0B;   /* Amber-500 */
--blitz-bg: #78350F;       /* Amber-900, subtle */
--full-accent: #3B82F6;    /* Blue-500 */
--full-bg: #1E3A5F;        /* Blue-900, subtle */
```

---

## Edge Cases

### No Chapters Detected

If PDF parsing couldn't detect chapters:

- **Fallback:** Single chapter, Full Mode only
- **No Blitz:** L1/L2 summaries not available
- **UI:** Mode toggle hidden, plays as single long chapter

```
┌─────────────────────────────────────────────────────┐
│               Deep Work                             │
│     📖 Full Document                                │
│                                                     │
│     Chapters could not be detected.                 │
│     Playing as single continuous audio.             │
│                                                     │
│   0:00  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  4:30:00   │
└─────────────────────────────────────────────────────┘
```

### Audio Not Yet Generated

If user tries to Go Deep but full chapter audio isn't ready:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│     🔍 Go Deep                                      │
│                                                     │
│     Full chapter audio is generating...             │
│                                                     │
│     ████████████░░░░░░░░░░░░  Ch 4: 45%            │
│                                                     │
│     [Notify Me When Ready]   [Continue Blitz]      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Options:**
- Continue listening in Blitz
- Get push notification when ready
- Queue the Go Deep action

### Partial Audio Generation

If some chapters have full audio, others don't:

```
Chapter 1: ⚡ Blitz | 📖 Full ✓
Chapter 2: ⚡ Blitz | 📖 Full ✓
Chapter 3: ⚡ Blitz | 📖 Full ⏳ generating
Chapter 4: ⚡ Blitz | 📖 Full ○ not started
```

- Blitz always available (summaries generated in Stage 1)
- Full requires Stage 2 audio generation
- Go Deep shows "generating" if not ready

---

## State Persistence

### What's Saved Per-Project

```json
{
  "project_id": "uuid",
  "listening_mode": "blitz",
  "current_chapter_id": "uuid",
  "current_content": "l2_kp2",
  "position_ms": 23000,
  "stay_in_full_mode": false,
  "chapters_completed": ["ch1", "ch2", "ch3"]
}
```

### Resume Behavior

When user returns to project:

1. **Same mode** as when they left
2. **Same position** (to the second)
3. **Same "stay in full" setting**

---

## API Interactions

### Fetch Chapter Content

```http
GET /api/projects/{id}/chapters/{chapter_id}
```

**Response:**
```json
{
  "id": "uuid",
  "number": 4,
  "title": "Deep Work Is Meaningful",
  "blitz": {
    "l1_summary": {
      "audio_url": "https://cdn.../l1.aac",
      "duration_ms": 60000,
      "text": "This chapter explores..."
    },
    "key_points": [
      {
        "number": 1,
        "title": "The Attention Residue Problem",
        "audio_url": "https://cdn.../kp1.aac",
        "duration_ms": 45000,
        "full_chapter_timestamp_ms": 0
      },
      {
        "number": 2,
        "title": "Why Depth Matters",
        "audio_url": "https://cdn.../kp2.aac",
        "duration_ms": 45000,
        "full_chapter_timestamp_ms": 480000
      }
    ]
  },
  "full": {
    "audio_url": "https://cdn.../full.aac",
    "duration_ms": 1920000,
    "status": "ready",
    "key_point_markers": [
      {"number": 1, "timestamp_ms": 0},
      {"number": 2, "timestamp_ms": 480000},
      {"number": 3, "timestamp_ms": 960000},
      {"number": 4, "timestamp_ms": 1440000}
    ]
  }
}
```

### Update Mode

```http
PUT /api/projects/{id}/progress
Content-Type: application/json

{
  "listening_mode": "full",
  "stay_in_full_mode": true
}
```

---

## Accessibility

| Element | A11y |
|---------|------|
| Mode toggle | `role="radiogroup"` with `aria-checked` |
| Go Deep button | `aria-label="Go deep into full chapter"` |
| Key point indicator | `aria-label="Key point 2 of 4"` |
| Progress (Blitz) | Announces "Chapter summary" or "Key point N" |
| Mode switch | Announces "Switched to Full Mode" |

### Screen Reader Announcements

| Event | Announcement |
|-------|--------------|
| Mode switch | "Now playing in [Blitz/Full] mode" |
| Go Deep | "Going deep, playing full chapter from key point 2" |
| Chapter end | "Chapter 4 complete" |
| Return to Blitz | "Returning to Blitz mode, next chapter summary" |

---

## Success Metrics

| Metric | Target | Description |
|--------|--------|-------------|
| Go Deep rate | >20% | % of L2 plays that trigger Go Deep |
| Mode switch rate | >30% | % of sessions with at least one mode switch |
| Blitz completion | >60% | % of Blitz chapters fully listened |
| Full completion | >40% | % of Full chapters fully listened |
| Return to Blitz | >50% | % of Go Deep users who return to Blitz after |

---

## User Research Questions

1. Do users understand the Blitz/Full distinction immediately?
2. Is "Go Deep" discoverable enough?
3. Should "Stay in Full Mode" be more prominent?
4. Is the position mapping (Blitz ↔ Full) accurate enough?

---

## Open Questions

1. **Go Deep from L1?** Allow Go Deep from chapter summary too?
2. **Preview before Go Deep?** Show estimated time for full chapter?
3. **Quick sample?** "Listen to 30 sec of full" before committing?
4. **Visual timeline?** Show Blitz vs Full coverage on progress bar?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
