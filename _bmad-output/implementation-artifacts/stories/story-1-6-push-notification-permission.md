# Story 1.6: Push Notification Permission

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 3  
**Created:** 2026-04-12

---

## User Story

As a **user**,  
I want to **be prompted for push notification permission**,  
So that I can **receive "audio ready" alerts**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR58 | Push notification opt-in prompt |
| FR13 | Notify when audio generation completes |

---

## Acceptance Criteria

### AC1: Contextual Permission Prompt

- **Given** I have uploaded my first content
- **When** processing begins
- **Then** I see a prompt to enable push notifications
- **And** the prompt appears after processing starts (not immediately)

### AC2: Clear Value Proposition

- **Given** I see the notification prompt
- **When** I read the message
- **Then** I understand the benefit: "We'll notify you when your audio is ready"
- **And** I see an example notification preview

### AC3: Enable Notifications

- **Given** I tap "Enable Notifications"
- **When** the browser permission dialog appears
- **Then** I can grant permission
- **And** my preference is saved to my account

### AC4: Dismiss Option

- **Given** I see the notification prompt
- **When** I tap "Not now"
- **Then** the prompt dismisses
- **And** I can enable later from settings
- **And** I won't be prompted again for 7 days

### AC5: Already Granted/Denied

- **Given** I have already granted or denied browser permissions
- **When** I would normally see the prompt
- **Then** the custom prompt does not appear
- **And** my existing permission is respected

### AC6: Notification Received

- **Given** I have enabled notifications
- **When** my audio generation completes
- **Then** I receive a push notification
- **And** tapping it opens the project

---

## Technical Design

### Push Notification Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        PUSH NOTIFICATION FLOW                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  [Upload] ──► [Processing Starts] ──► [Show Custom Prompt]              │
│                                               │                          │
│                            ┌──────────────────┴──────────────────┐      │
│                            ▼                                      ▼      │
│                     [Enable]                                [Not Now]    │
│                        │                                         │       │
│                        ▼                                         ▼       │
│              [Browser Permission]                    [Save Dismissal]    │
│                        │                                                 │
│                        ▼                                                 │
│              [Register Service Worker]                                   │
│                        │                                                 │
│                        ▼                                                 │
│              [Subscribe to Push]                                         │
│                        │                                                 │
│                        ▼                                                 │
│              [Save Subscription to Server]                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Components to Create

| Component | File | Description |
|-----------|------|-------------|
| NotificationPrompt | `frontend/src/components/NotificationPrompt.tsx` | Custom prompt UI |
| PushService | `frontend/src/services/push.ts` | Push subscription logic |
| Service Worker | `frontend/public/sw.js` | Handles push events |

### Backend: Push Subscription

```sql
CREATE TABLE push_subscriptions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id),
  endpoint VARCHAR(500) NOT NULL,
  p256dh VARCHAR(200) NOT NULL,
  auth VARCHAR(100) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(user_id, endpoint)
);
```

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.6.1** Create push_subscriptions table migration
- [ ] **1.6.2** Create `POST /api/push/subscribe` endpoint
- [ ] **1.6.3** Create `DELETE /api/push/unsubscribe` endpoint
- [ ] **1.6.4** Integrate web-push library for sending
- [ ] **1.6.5** Send push on audio generation complete
- [ ] **1.6.6** Unit tests for push endpoints

### Frontend Tasks

- [ ] **1.6.7** Create NotificationPrompt component
- [ ] **1.6.8** Create PushService with subscription logic
- [ ] **1.6.9** Add service worker for push handling
- [ ] **1.6.10** Add notification permission check
- [ ] **1.6.11** Add "dismissed_at" tracking in localStorage
- [ ] **1.6.12** Add settings toggle for notifications
- [ ] **1.6.13** Unit tests for NotificationPrompt
- [ ] **1.6.14** E2E test for push flow (mocked)

---

## API Contracts

### Endpoint: POST /api/push/subscribe

**Request:**
```json
{
  "subscription": {
    "endpoint": "https://fcm.googleapis.com/...",
    "keys": {
      "p256dh": "base64...",
      "auth": "base64..."
    }
  }
}
```

**Response:** HTTP 201
```json
{
  "id": "uuid",
  "created_at": "2026-04-12T14:30:00Z"
}
```

---

## UI Wireframes

### Custom Permission Prompt

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  🔔 Get notified when your audio is ready   │   │
│  │                                             │   │
│  │  ┌─────────────────────────────────────┐   │   │
│  │  │ Disona                        now   │   │   │
│  │  │ "Deep Work" is ready to listen!    │   │   │
│  │  └─────────────────────────────────────┘   │   │
│  │           ↑ Example notification            │   │
│  │                                             │   │
│  │  ┌───────────────┐  ┌───────────────┐      │   │
│  │  │ Enable        │  │ Not now       │      │   │
│  │  └───────────────┘  └───────────────┘      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Push Notification Content

### Audio Ready Notification

```json
{
  "title": "Disona",
  "body": "\"Deep Work\" is ready to listen!",
  "icon": "/icons/icon-192.png",
  "badge": "/icons/badge-72.png",
  "data": {
    "url": "/projects/uuid",
    "project_id": "uuid"
  }
}
```

---

## Definition of Done

- [ ] Custom prompt shows at right time
- [ ] Browser permission requested correctly
- [ ] Subscription saved to server
- [ ] Push notification received when audio ready
- [ ] Tapping notification opens project
- [ ] "Not now" dismisses for 7 days
- [ ] Works in Chrome, Firefox, Edge (Safari limited)

---

## Browser Support

| Browser | Push Support |
|---------|-------------|
| Chrome | ✅ Full |
| Firefox | ✅ Full |
| Edge | ✅ Full |
| Safari | ⚠️ Limited (macOS only) |
| iOS Safari | ❌ Not supported |

**Note:** iOS PWA push requires app store distribution (post-MVP)

---

## Dependencies

| Dependency | Status |
|------------|--------|
| Service Worker (PWA) | ✅ Basic setup exists |
| Story 3.4 (Generation Complete) | Backlog |
| VAPID keys | 🔴 Need to generate |

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 3.4 Generation Notification | Triggers this notification |
| 1.5 Guided First Upload | Shows prompt after upload |
