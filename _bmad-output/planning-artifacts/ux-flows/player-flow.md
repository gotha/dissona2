# Player Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

The player is the core experience of Disona. It has two states: **Mini Player** (persistent bar) and **Full Player** (expanded view). Users can seamlessly transition between them and control playback from either state.

---

## Mini Player

Persistent bar at the bottom of the screen when audio is playing.

### Layout

```
┌─────────────────────────────────────────────────────┐
│  📘  Deep Work • Ch 4: Deep Work Is Rare     advancement advancement ││  ██████████████░░░░░░░░░░░░░░░░░░░░░  12:34 / 28:00 │
└─────────────────────────────────────────────────────┘
     │         │                              │    │
     │         │                              │    └─ Skip forward
     │         │                              └────── Play/Pause
     │         └───────────────────────────────────── Title + Chapter
     └─────────────────────────────────────────────── Cover/Icon
```

### Compact Layout (< 360px width)

```
┌─────────────────────────────────┐
│  📘  Deep Work Ch4    ▶  ▷▷    │
│  ████████░░░░░░░░░░░  12:34    │
└─────────────────────────────────┘
```

### Mini Player Elements

| Element | Description |
|---------|-------------|
| **Cover** | Book icon/cover image (40x40px) |
| **Title** | Project title, truncated |
| **Chapter** | Current chapter name, truncated |
| **Progress Bar** | Thin seekable bar |
| **Time** | Current position / total duration |
| **Play/Pause** | Toggle playback |
| **Skip** | +15 seconds |

### Mini Player Interactions

| Action | Result |
|--------|--------|
| Tap anywhere (except buttons) | Expand to Full Player |
| Tap Play/Pause | Toggle playback |
| Tap Skip | Jump +15 seconds |
| Swipe up | Expand to Full Player |
| Swipe left | Dismiss (stop playback) |
| Drag progress bar | Seek to position |

---

## Full Player — Expanded View

### Layout

```
┌─────────────────────────────────────────────────────┐
│  ↓                              ⋮                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│                    ┌─────────┐                      │
│                    │         │                      │
│                    │   📘    │                      │
│                    │         │                      │
│                    └─────────┘                      │
│                                                     │
│               Deep Work                             │
│         Chapter 4: Deep Work Is Rare                │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│   12:34  ████████████░░░░░░░░░░░░░░░░░░  28:00    │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│       ⏮️        ◀◀        ▶️        ▶▶        ⏭️     │
│      prev      -15s     play     +15s     next     │
│      seg                                  seg      │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│   [⚡ Blitz]  [📖 Full]           1.5x    📥  📤   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Full Player Elements

| Element | Description |
|---------|-------------|
| **Collapse** | Down arrow to minimize to mini player |
| **Menu** | Options (share, download, etc.) |
| **Cover** | Large book cover/icon (150x150px) |
| **Title** | Project title |
| **Chapter** | Current chapter name |
| **Progress Bar** | Full-width seekable bar |
| **Time** | Current / Total with both visible |
| **Transport** | 5-button playback control |
| **Mode Toggle** | Blitz ⚡ / Full 📖 switch |
| **Speed** | Playback speed control |
| **Download** | Download for offline |
| **Share** | Share this chapter |

---

## Transport Controls (5 Buttons)

```
    ⏮️           ◀◀           ▶️           ▶▶           ⏭️
   Prev        Rewind       Play/       Forward       Next
   Segment      15s        Pause         15s        Segment
```

| Button | Action | Long Press |
|--------|--------|------------|
| ⏮️ Prev | Previous segment/key point | Previous chapter |
| ◀◀ -15s | Rewind 15 seconds | Continuous rewind |
| ▶️ Play | Toggle play/pause | — |
| ▶▶ +15s | Forward 15 seconds | Continuous forward |
| ⏭️ Next | Next segment/key point | Next chapter |

### Segment Navigation

In **Blitz Mode:**
- Prev/Next moves between L2 key point summaries
- After last L2, moves to next L1 (chapter summary)

In **Full Mode:**
- Prev/Next moves between segments within chapter
- After last segment, moves to next chapter

---

## Expand/Collapse Animation

### Mini → Full (Expand)

```
Frame 1 (0ms):     Mini player at bottom
Frame 2 (100ms):   Cover begins scaling up
Frame 3 (200ms):   Controls fade in
Frame 4 (300ms):   Full player complete
```

**Animation:**
- Duration: 300ms
- Easing: ease-out
- Cover scales from 40px → 150px
- Background slides up from bottom
- Controls fade in (opacity 0 → 1)

### Full → Mini (Collapse)

```
Frame 1 (0ms):     Full player visible
Frame 2 (100ms):   Controls fade out
Frame 3 (200ms):   Cover scales down
Frame 4 (300ms):   Mini player at bottom
```

**Triggers:**
- Tap collapse button (↓)
- Swipe down gesture
- Navigate to another screen

---

## Mode Toggle: Blitz ⚡ vs Full 📖

### Toggle UI

```
┌──────────────────────────────────┐
│   [⚡ Blitz]    [📖 Full]        │
│       ▲                          │
│    active                        │
└──────────────────────────────────┘
```

### Mode Behavior

| Mode | Content Played | Navigation |
|------|----------------|------------|
| **Blitz ⚡** | L1 summary → L2 summaries | Summaries only |
| **Full 📖** | Full chapter narration | Complete content |

### Mode Switch Behavior

**Blitz → Full:**
- If on L1 summary: Jump to chapter start (0:00)
- If on L2 summary: Jump to corresponding position in full chapter

**Full → Blitz:**
- Find nearest L2 key point to current position
- Resume from that L2 summary

### Go Deep Button (Blitz Mode Only)

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│         Chapter 4: Deep Work Is Rare                │
│         Key Point 2: The Attention Residue         │
│                                                     │
│              ┌─────────────────┐                    │
│              │  🔍 Go Deep     │                    │
│              └─────────────────┘                    │
│                                                     │
│          Hear the full chapter from here            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Go Deep:**
- Appears during L2 key point playback
- Tap → Switch to Full Mode at corresponding timestamp
- After chapter ends → Return to Blitz (next L1)
- "Stay in Full Mode" checkbox available

---

## Speed Control

### Speed Button

```
┌───────┐
│  1.5x │  ← Tap to open speed picker
└───────┘
```

### Speed Picker

```
┌─────────────────────────┐
│  Playback Speed         │
├─────────────────────────┤
│  0.5x                   │
│  0.75x                  │
│  1x     ← default       │
│  1.25x                  │
│  1.5x   ✓ selected      │
│  1.75x                  │
│  2x                     │
│  2.5x                   │
│  3x                     │
└─────────────────────────┘
```

**Speed presets:** 0.5x, 0.75x, 1x, 1.25x, 1.5x, 1.75x, 2x, 2.5x, 3x

**Persistence:** Speed is saved per-user (not per-project)

---

## Chapter Browser

Accessible via menu or swipe gesture:

```
┌─────────────────────────────────────────────────────┐
│  ← Chapters                                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. Introduction                    ✓ 15:00        │
│  2. Deep Work Is Valuable           ✓ 28:00        │
│  3. Deep Work Is Rare               ✓ 22:00        │
│  4. Deep Work Is Meaningful         ▶ 12:34/25:00  │
│  5. Rule #1: Work Deeply            ○ 35:00        │
│  6. Rule #2: Embrace Boredom        ○ 30:00        │
│  ...                                               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Chapter States:**
- ✓ Completed
- ▶ Currently playing (with position)
- ○ Not started

**Tap chapter** → Jump to that chapter

### Blitz Mode Chapter Browser

Shows L1/L2 hierarchy:

```
┌─────────────────────────────────────────────────────┐
│  4. Deep Work Is Meaningful                         │
│     ├─ L1: Chapter Summary           ▶ playing     │
│     ├─ L2: Why Depth Matters         ○             │
│     ├─ L2: The Craftsman Mindset     ○             │
│     ├─ L2: Neurological Argument     ○             │
│     └─ L2: Psychological Argument    ○             │
│                                                     │
│  5. Rule #1: Work Deeply                           │
│     ├─ L1: Chapter Summary           ○             │
│     ...                                            │
└─────────────────────────────────────────────────────┘
```

---

## Progress Bar Interactions

### Seeking

```
┌─────────────────────────────────────────────────────┐
│   12:34  ████████████░░░░░░░░░░░░░░░░░░  28:00     │
│                    ▲                                │
│              drag to seek                           │
└─────────────────────────────────────────────────────┘
```

**Interactions:**
- Tap anywhere on bar → Jump to position
- Drag thumb → Scrub through content
- While dragging → Show preview time

### Key Point Markers (Full Mode)

```
┌─────────────────────────────────────────────────────┐
│   ████████│███│█████│░░░░│░░░░░░│░░░░░░░░░░░░░░    │
│           ▲   ▲     ▲    ▲      ▲                  │
│          KP1 KP2   KP3  KP4    KP5                 │
└─────────────────────────────────────────────────────┘
```

**Key point markers:**
- Small vertical lines on progress bar
- Show where each key point starts
- Tap marker → Jump to that key point

---

## Lock Screen / MediaSession

Uses MediaSession API for lock screen controls:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                    ┌─────────┐                      │
│                    │   📘    │                      │
│                    └─────────┘                      │
│                                                     │
│               Deep Work                             │
│         Chapter 4: Deep Work Is Rare                │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│       ⏮️        ◀◀        ▶️        ▶▶        ⏭️     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**MediaSession metadata:**
- Title: Chapter name
- Artist: Project title
- Album: "Disona"
- Artwork: Book cover

**MediaSession actions:**
- play, pause
- previoustrack (prev segment)
- nexttrack (next segment)
- seekbackward (-15s)
- seekforward (+15s)

---

## Live Transcript (Optional)

In expanded player, show synchronized transcript:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  The key to developing deep work habits is to      │
│  move beyond good intentions and add routines      │
│  and rituals to your working life designed to      │
│  minimize the amount of your limited willpower     │
│  necessary to transition into and maintain a       │
│  state of unbroken concentration.                  │
│                                                     │
│  ▶ This strategy suggests that you eliminate       │ ← current
│  ▶ or radically minimize shallow work from         │ ← sentence
│  ▶ your schedule.                                  │
│                                                     │
│  The goal is not to eliminate all shallow work     │
│  but to make sure that shallow work doesn't        │
│  dominate your time and attention.                 │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Behavior:**
- Auto-scrolls to keep current sentence visible
- Current sentence highlighted
- Tap sentence → Jump to that position
- Manual scroll pauses auto-scroll (resumes on playback progress)

---

## Player States

### Loading

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                    ◌ Loading...                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Buffering

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│       ⏮️        ◀◀        ◌        ▶▶        ⏭️     │
│                        buffering                    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Error

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│              ⚠️ Playback Error                      │
│                                                     │
│         Couldn't load audio. Check your            │
│         connection and try again.                  │
│                                                     │
│                  [Retry]                            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## End of Chapter / Book

### End of Chapter

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│           ✓ Chapter Complete                        │
│                                                     │
│      Chapter 4: Deep Work Is Meaningful             │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  ▶ Next Chapter     │                     │
│         └─────────────────────┘                     │
│                                                     │
│         Playing next in 5s...                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Auto-advance:** Plays next chapter after 5 seconds (configurable)

### End of Book

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│           🎉 Book Complete!                         │
│                                                     │
│              Deep Work                              │
│           12 chapters • 4h 30m                      │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  📤 Share Book      │                     │
│         └─────────────────────┘                     │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  📚 Back to Library │                     │
│         └─────────────────────┘                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**No auto-play:** Stops at end of book, does not auto-play next book

---

## Gestures Summary

| Gesture | Location | Action |
|---------|----------|--------|
| Tap | Mini player | Expand to full |
| Swipe up | Mini player | Expand to full |
| Swipe left | Mini player | Dismiss player |
| Tap ↓ | Full player | Collapse to mini |
| Swipe down | Full player | Collapse to mini |
| Tap progress bar | Either | Seek to position |
| Drag progress bar | Either | Scrub through content |
| Double tap center | Full player | Play/Pause |
| Double tap left | Full player | -15 seconds |
| Double tap right | Full player | +15 seconds |

---

## API Interactions

### Update Progress

```http
PUT /api/projects/{id}/progress
Content-Type: application/json

{
  "chapter_id": "uuid",
  "position_ms": 123456,
  "listening_mode": "blitz"
}
```

**Frequency:** Every 10 seconds during playback + on pause/stop

### Fetch Audio URL

```http
GET /api/audio/{audio_file_id}
```

**Response:**
```json
{
  "url": "https://cdn.disona.io/audio/uuid.aac",
  "duration_ms": 1680000,
  "expires_at": "2026-04-12T12:00:00Z"
}
```

---

## Accessibility

| Element | A11y |
|---------|------|
| Play/Pause | `aria-label="Play"` / `"Pause"` |
| Progress bar | `role="slider"` with `aria-valuenow` |
| Speed button | `aria-label="Playback speed 1.5x"` |
| Mode toggle | `role="radiogroup"` |
| Transport buttons | All have `aria-label` |
| Mini player | `role="region"` `aria-label="Audio player"` |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Expand animation | <300ms |
| Play latency | <500ms |
| Seek latency | <200ms |
| Lock screen sync | <1s |
| Progress save | 100% reliable |

---

## Open Questions

1. **Sleep timer:** Add for MVP or defer?
2. **Bookmarks:** Allow marking positions?
3. **Transcript:** Include in MVP or defer?
4. **Queue view:** Accessible from player or library only?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
