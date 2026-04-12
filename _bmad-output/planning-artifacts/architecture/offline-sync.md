# Offline Sync Specification

## Overview

Disona is a PWA that supports full offline playback of downloaded content.

---

## Offline Capabilities

| Feature | Offline Support | Storage |
|---------|-----------------|---------|
| Audio playback | ✅ Yes (downloaded) | Cache API |
| Playback controls | ✅ Yes | N/A |
| Progress tracking | ✅ Yes (local) | IndexedDB |
| Library browsing | ✅ Yes (cached) | IndexedDB |
| Go Deep transitions | ✅ Yes (if downloaded) | Cache API |
| New uploads | ❌ No | N/A |
| Generate audio | ❌ No | N/A |

---

## Storage Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              PWA (Browser)                                   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                         SERVICE WORKER                               │    │
│  │                                                                      │    │
│  │  • Intercepts all requests                                          │    │
│  │  • Serves cached content when offline                               │    │
│  │  • Queues sync operations                                           │    │
│  │  • Background sync when online                                       │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│          ┌─────────────────────────────────────────────┐                    │
│          │                                             │                    │
│          ▼                         ▼                   ▼                    │
│  ┌───────────────┐       ┌─────────────────┐   ┌─────────────────┐         │
│  │  CACHE API    │       │   IndexedDB     │   │  LocalStorage   │         │
│  │               │       │                 │   │                 │         │
│  │ • Audio files │       │ • Projects      │   │ • Auth token    │         │
│  │ • App shell   │       │ • Chapters      │   │ • User prefs    │         │
│  │ • Static      │       │ • Progress      │   │                 │         │
│  │   assets      │       │ • Pending syncs │   │                 │         │
│  └───────────────┘       └─────────────────┘   └─────────────────┘         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Download Options

| Mode | Size | Includes |
|------|------|----------|
| **Blitz** | ~80 MB | L1 + L2 summaries |
| **Full** | ~375 MB | Full narration only |
| **All** | ~450 MB | Everything |

---

## Progress Sync

### When Online

1. User plays audio
2. Update local IndexedDB (immediate)
3. Debounce 5 seconds
4. POST /api/playback/progress (background)
5. Mark as synced in IndexedDB

### When Offline

1. User plays audio
2. Update local IndexedDB (immediate)
3. Mark as unsynced
4. Queue in pendingSyncs
5. [When online] Background sync via Service Worker
6. Mark as synced

---

## Conflict Resolution

**Strategy:** Last Write Wins (with timestamp)

When syncing offline progress:
1. Client sends progress with `updatedAt` timestamp
2. Server compares with current `updatedAt`
3. Only update if incoming timestamp is newer
4. Return current state if server has newer data
5. Client updates local state if server wins

---

## Storage Management

### Quota Monitoring

- Check storage usage via `navigator.storage.estimate()`
- Warn user when approaching limits

### Auto-Cleanup

- When storage > 80% full
- Remove oldest downloads first (by last played)
- Continue until < 60% full

---

## Service Worker Strategies

| Content Type | Strategy | Cache Name |
|--------------|----------|------------|
| App shell | Cache first, update in background | `app-shell-v1` |
| API calls | Network first, fallback to cache | `api-v1` |
| Audio files | Cache only (explicit download) | `audio-v1` |

---

## IndexedDB Schema

```typescript
interface OfflineDB {
  projects: {
    id: string;
    title: string;
    chapters: Chapter[];
    downloaded: boolean;
    downloadMode: 'blitz' | 'full' | 'all';
    downloadedAt: Date;
    lastPlayedAt: Date;
  };
  
  progress: {
    projectId: string;
    chapterId: string;
    audioType: string;
    positionMs: number;
    updatedAt: Date;
    synced: boolean;
  };
  
  pendingSyncs: {
    id: string;
    type: 'progress';
    payload: any;
    createdAt: Date;
  };
}
```

---

## User Experience

### Download States

- **Not downloaded** — Show download button with size options
- **Downloading** — Show progress bar with cancel option
- **Downloaded** — Show checkmark, remove option, last sync time

### Offline Indicator

- Banner when offline: "You're offline. Progress will sync when back online."
- Auto-dismiss when back online
