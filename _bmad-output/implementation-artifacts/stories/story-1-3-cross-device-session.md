# Story 1.3: Cross-Device Session

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 3  
**Created:** 2026-04-12

---

## User Story

As a **user with multiple devices**,  
I want to **stay logged in across my phone and computer**,  
So that I can **seamlessly switch between devices**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR46 | Session persists across devices |
| NFR-R4 | Playback sync within 5 seconds |

---

## Acceptance Criteria

### AC1: Same Account Across Devices

- **Given** I am logged in on my phone
- **When** I open Disona on my computer and sign in with same Google account
- **Then** I see the same account on both devices
- **And** I see the same library with all my projects

### AC2: Playback Position Sync

- **Given** I am listening on my phone at position 12:34
- **When** I pause and open Disona on my computer
- **Then** my playback position syncs within 5 seconds
- **And** I can resume from 12:34 on my computer

### AC3: Real-Time Progress Update

- **Given** I have Disona open on two devices
- **When** I make progress on device A
- **Then** device B shows updated progress within 5 seconds
- **And** no manual refresh is required

### AC4: Independent Playback

- **Given** I am listening on device A
- **When** I start listening on device B
- **Then** device A continues playing (no forced stop)
- **And** the most recent pause position is saved

### AC5: Offline Sync

- **Given** I was offline on my phone
- **When** I come back online
- **Then** my offline progress syncs to server
- **And** appears on other devices

---

## Technical Design

### Sync Architecture

```
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│   Phone     │────────►│   API       │◄────────│  Computer   │
│   (PWA)     │         │   Service   │         │   (PWA)     │
└─────────────┘         └──────┬──────┘         └─────────────┘
                               │
                               ▼
                        ┌─────────────┐
                        │  PostgreSQL │
                        │  (progress) │
                        └─────────────┘
```

### Progress Sync Strategy

1. **Write:** Save progress to server every 10 seconds during playback
2. **Read:** Fetch latest progress on app focus/visibility change
3. **Conflict:** Last-write-wins (most recent timestamp)

### Components to Modify

| Component | File | Changes |
|-----------|------|---------|
| Progress API | `services/api/src/handlers/progress.rs` | Add sync endpoint |
| Progress Store | `frontend/src/stores/progressStore.ts` | Add sync logic |
| Player | `frontend/src/components/Player.tsx` | Periodic save |
| App | `frontend/src/App.tsx` | Visibility change listener |

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.3.1** Create `GET /api/projects/:id/progress` endpoint
- [ ] **1.3.2** Create `PUT /api/projects/:id/progress` endpoint
- [ ] **1.3.3** Add `updated_at` timestamp to progress records
- [ ] **1.3.4** Unit tests for progress endpoints

### Frontend Tasks

- [ ] **1.3.5** Create progressStore with sync logic
- [ ] **1.3.6** Add periodic progress save (every 10s)
- [ ] **1.3.7** Add visibility change listener for sync
- [ ] **1.3.8** Add offline queue for progress updates
- [ ] **1.3.9** Unit tests for sync logic
- [ ] **1.3.10** E2E test for cross-device sync (mocked)

---

## API Contracts

### Endpoint: GET /api/projects/:id/progress

**Response:** HTTP 200
```json
{
  "project_id": "uuid",
  "chapter_id": "uuid",
  "position_ms": 754000,
  "listening_mode": "blitz",
  "updated_at": "2026-04-12T14:30:00Z"
}
```

### Endpoint: PUT /api/projects/:id/progress

**Request:**
```json
{
  "chapter_id": "uuid",
  "position_ms": 754000,
  "listening_mode": "blitz"
}
```

**Response:** HTTP 200
```json
{
  "updated_at": "2026-04-12T14:30:05Z"
}
```

---

## Definition of Done

- [ ] Progress syncs across devices within 5 seconds
- [ ] Offline progress queued and synced when online
- [ ] No data loss during sync conflicts
- [ ] Unit and E2E tests pass

---

## Dependencies

| Dependency | Status |
|------------|--------|
| Story 1.1 (OAuth) | Ready for Dev |
| Story 1.2 (Sessions) | Ready for Dev |
| API service | ✅ Scaffolded |

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 4.6 Progress Persistence | Uses same sync mechanism |
| 4.8 Download for Offline | Offline queue shared |
