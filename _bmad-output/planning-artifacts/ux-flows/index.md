# UX Flow Specifications — Index

**Project:** Disona  
**Last Updated:** 2026-04-12

---

## Overview

This directory contains detailed UX specifications for all major user flows in Disona. Each spec includes wireframes, state diagrams, interaction patterns, API contracts, and accessibility considerations.

---

## Flow Specifications

| Flow | Description | Status |
|------|-------------|--------|
| [Upload Flow](./upload-flow.md) | File upload, processing, project creation | ✅ Complete |
| [Library Flow](./library-flow.md) | Home screen, project list, filtering | ✅ Complete |
| [Player Flow](./player-flow.md) | Mini player, full player, controls | ✅ Complete |
| [Blitz & Full Mode](./blitz-full-mode-flow.md) | Depth on demand, Go Deep, mode switching | ✅ Complete |
| [Project Detail](./project-detail-flow.md) | Project view, chapters, audio generation | ✅ Complete |
| [Share Flow](./share-flow.md) | Email-locked sharing, recipient experience | ✅ Complete |

---

## User Journey Map

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              USER JOURNEY                                    │
│                                                                              │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│   │  Login   │───►│  Library │───►│  Upload  │───►│ Project  │             │
│   └──────────┘    └────┬─────┘    └──────────┘    │  Detail  │             │
│                        │                          └────┬─────┘             │
│                        │                               │                    │
│                        │         ┌─────────────────────┤                    │
│                        │         │                     │                    │
│                        │         ▼                     ▼                    │
│                        │    ┌──────────┐         ┌──────────┐              │
│                        │    │ Generate │         │  Player  │              │
│                        │    │  Audio   │         │  (Mini)  │              │
│                        │    └────┬─────┘         └────┬─────┘              │
│                        │         │                     │                    │
│                        │         ▼                     ▼                    │
│                        │    ┌──────────┐         ┌──────────┐              │
│                        └───►│  Player  │◄────────│  Player  │              │
│                             │  (Full)  │         │  (Full)  │              │
│                             └────┬─────┘         └──────────┘              │
│                                  │                                          │
│                    ┌─────────────┼─────────────┐                           │
│                    │             │             │                            │
│                    ▼             ▼             ▼                            │
│              ┌──────────┐  ┌──────────┐  ┌──────────┐                      │
│              │  Blitz   │  │   Full   │  │   Share  │                      │
│              │   Mode   │◄─┤   Mode   │  │   Flow   │                      │
│              └────┬─────┘  └──────────┘  └──────────┘                      │
│                   │              ▲                                          │
│                   │  Go Deep     │                                          │
│                   └──────────────┘                                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Reference

### Entry Points

| Screen | Entry From |
|--------|------------|
| Library | Login, App launch |
| Upload | Library (FAB) |
| Project Detail | Library tap, Upload complete |
| Player (Mini) | Any playback trigger |
| Player (Full) | Mini player expand, Project play |
| Share | Project Detail, Player |

### Key States Per Flow

| Flow | Key States |
|------|------------|
| Upload | Selecting → Uploading → Processing → Complete |
| Library | Empty, Loading, Populated, Filtered |
| Player | Mini, Full, Loading, Buffering, Error |
| Blitz/Full | L1 Summary, L2 Key Point, Full Narration |
| Project | Processing, Ready, Generating, Audio Ready, Failed |
| Share | Compose, Sent, Recipient Auth, Recipient View |

---

## Design Principles

### 1. Progressive Disclosure
- Start simple, reveal complexity on demand
- Blitz Mode as default, Full Mode available
- Chapter list collapsed, expandable

### 2. Immediate Feedback
- Upload progress visible instantly
- Processing status updates every 3 seconds
- Playback responds < 500ms

### 3. Error Recovery
- Failed uploads can retry
- Failed parsing shows actionable error
- Offline gracefully degrades

### 4. Consistent Patterns
- Progress bars everywhere use same style
- Play button always green
- Error states always red/warning

---

## Component Library Reference

### Buttons

| Type | Usage |
|------|-------|
| Primary (filled) | Main CTA: Play, Generate, Share |
| Secondary (outline) | Alternative: Cancel, Skip |
| Danger (red) | Destructive: Delete, Revoke |
| Icon | Compact actions: +15s, Download |

### Progress Indicators

| Type | Usage |
|------|-------|
| Linear bar | Upload, playback, generation |
| Circular | Loading states |
| Dot sequence | Blitz mode chapter position |

### Cards

| Type | Usage |
|------|-------|
| Project card | Library items |
| Chapter card | Project detail list |
| Share card | Shared With Me |

---

## Accessibility Checklist

All flows implement:

- [ ] Semantic HTML (`role`, `aria-*`)
- [ ] Focus management for modals
- [ ] Screen reader announcements for state changes
- [ ] Touch targets ≥ 48px
- [ ] Color contrast ≥ 4.5:1
- [ ] Keyboard navigation
- [ ] Reduced motion support

---

## Related Documents

| Document | Location |
|----------|----------|
| PRD | [../prd.md](../prd.md) |
| Architecture | [../architecture.md](../architecture.md) |
| UX Design (High-level) | [../ux-design.md](../ux-design.md) |
| Database Schema | [../architecture/database.md](../architecture/database.md) |

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial index with 6 flow specs |
