# Frontend Architecture

## Tech Stack

| Layer | Technology | Why |
|-------|------------|-----|
| Framework | React 18 | Mature, large ecosystem |
| Language | TypeScript | Type safety |
| Build | Vite | Fast builds, good DX |
| Styling | Tailwind CSS | Utility-first, rapid development |
| State | Zustand | Simple, minimal boilerplate |
| Data Fetching | TanStack Query | Caching, background sync |
| Routing | React Router v6 | Standard, PWA-friendly |
| PWA | Vite PWA Plugin | Service worker, manifest |

---

## Directory Structure

```
frontend/
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   │
│   ├── components/
│   │   ├── ui/              # Base components
│   │   ├── layout/          # Layout components
│   │   ├── player/          # Audio player
│   │   └── project/         # Project components
│   │
│   ├── pages/               # Route pages
│   │
│   ├── stores/              # Zustand stores
│   │   ├── playerStore.ts
│   │   ├── authStore.ts
│   │   ├── downloadStore.ts
│   │   └── uiStore.ts
│   │
│   ├── hooks/               # Custom hooks
│   ├── api/                 # API client
│   ├── lib/                 # Utilities
│   └── types/               # TypeScript types
│
├── vite.config.ts
└── package.json
```

---

## State Stores

| Store | Purpose |
|-------|---------|
| `playerStore` | Playback state, queue, mode (blitz/full) |
| `authStore` | User session, token |
| `downloadStore` | Offline downloads, progress |
| `uiStore` | Modals, toasts, UI state |

---

## Audio Player System

```
┌─────────────────────────────────────────────────────────────────┐
│                      PlayerProvider                              │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     AudioEngine                             │ │
│  │  • HTMLAudioElement                                         │ │
│  │  • MediaSession API (lock screen)                          │ │
│  │  • Preload next track                                       │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                  PlayerStore (Zustand)                      │ │
│  │  • Current track, playback state                           │ │
│  │  • Queue, mode (blitz/full)                                │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│              ┌───────────────┼───────────────┐                  │
│              ▼               ▼               ▼                  │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────┐         │
│  │  PlayerBar    │ │ PlayerFull    │ │ LockScreen    │         │
│  │  (mini)       │ │ (expanded)    │ │ (MediaSession)│         │
│  └───────────────┘ └───────────────┘ └───────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Go Deep Feature

| Mode | Behavior |
|------|----------|
| **Temporary Deep** | Play full section, return to blitz |
| **Full Deep** | Switch to full mode for chapter |

Implementation:
1. Find current key point's position in full narration
2. Seek to `keyPoint.fullAudioStartMs`
3. If temporary, return to blitz after `fullAudioEndMs`

---

## PWA Features

| Feature | Implementation |
|---------|----------------|
| Offline playback | Cache API for audio files |
| Background play | Audio element + Service Worker |
| Lock screen | MediaSession API |
| Install prompt | PWA manifest |
| Progress sync | IndexedDB + background sync |

---

## Data Fetching

Using TanStack Query for:
- Automatic caching
- Background refetch
- Offline support
- Loading/error states

```typescript
// Stale times
Projects list: 5 minutes
Project detail: 1 minute
Manifest: 10 minutes (cached for offline)
```

---

## Key Components

| Component | Purpose |
|-----------|---------|
| `PlayerBar` | Mini player (bottom bar) |
| `PlayerFullscreen` | Expanded player view |
| `GoDeepButton` | Toggle summary ↔ full |
| `ProjectCard` | Project in library |
| `ProcessingStatus` | Upload/generation progress |
| `UploadDropzone` | Document upload |
| `ChapterList` | Chapter navigation |
