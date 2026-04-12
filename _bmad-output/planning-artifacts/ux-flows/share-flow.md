# Share Flow — UX Specification

**Status:** Draft  
**Author:** Gotha  
**Date:** 2026-04-12

---

## Overview

Sharing allows users to give others access to their audio content. Disona uses **email-locked sharing** — recipients must authenticate with the specified email to access shared content. This creates a personal, secure sharing experience.

---

## Share Types

| Type | What's Shared | Use Case |
|------|---------------|----------|
| **Project** | Entire book with all chapters | "Read this book" |
| **Chapter** | Single chapter | "Check out this chapter" |
| **Clip** | Segment within chapter | "Listen to this part" |

---

## Entry Points

| Location | Trigger | Default Share Type |
|----------|---------|-------------------|
| Project Detail | 📤 Share button | Project |
| Project Detail menu | Share option | Project |
| Player (full) | 📤 Share button | Current chapter |
| Player context menu | Share this part | Clip (current segment) |
| Chapter (expanded) | Share button | Chapter |

---

## Share Flow — Step by Step

### Step 1: Share Modal Opens

```
┌─────────────────────────────────────────────────────┐
│  Share                                       ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📘 Deep Work                                       │
│  12 chapters • 4h 30m                               │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  What to share:                                     │
│                                                     │
│  ● Entire project (12 chapters)                    │
│  ○ Current chapter only                            │
│  ○ Current clip (2:34)                             │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Next                              │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Step 2: Recipient Email

```
┌─────────────────────────────────────────────────────┐
│  Share                                       ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Who should receive this?                           │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  Email address                              │   │
│  │  sarah@example.com                          │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  🔒 Only this email can access the shared content   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Add a personal message (optional):                 │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  Hey Sarah! Check out this book about       │   │
│  │  focused work. Chapter 4 is especially      │   │
│  │  relevant to what we discussed.             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Share                             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Step 3: Share Confirmation

```
┌─────────────────────────────────────────────────────┐
│  Shared! ✓                                   ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│              ✓                                      │
│                                                     │
│  Deep Work has been shared with                     │
│  sarah@example.com                                  │
│                                                     │
│  They'll receive an email with access instructions. │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Share link (for manual sending):                   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  https://disona.io/s/abc123xyz              │   │
│  │                                    📋 Copy  │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ⚠️ This link only works for sarah@example.com     │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Done                              │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Recipient Experience

### Email Notification

```
┌─────────────────────────────────────────────────────┐
│  From: Disona <noreply@disona.io>                  │
│  To: sarah@example.com                             │
│  Subject: John shared "Deep Work" with you         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Hi Sarah,                                          │
│                                                     │
│  John shared an audiobook with you on Disona:      │
│                                                     │
│  📘 Deep Work                                       │
│  12 chapters • 4h 30m                               │
│                                                     │
│  Message from John:                                 │
│  "Hey Sarah! Check out this book about focused     │
│  work. Chapter 4 is especially relevant to what    │
│  we discussed."                                    │
│                                                     │
│         ┌─────────────────────┐                    │
│         │   Listen Now        │                    │
│         └─────────────────────┘                    │
│                                                     │
│  Or copy this link: https://disona.io/s/abc123     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Recipient Opens Link — Not Logged In

```
┌─────────────────────────────────────────────────────┐
│                     Disona                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│              📘 Deep Work                           │
│                                                     │
│         Shared by John                              │
│         12 chapters • 4h 30m                        │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  This content was shared with:                      │
│  sarah@example.com                                  │
│                                                     │
│  Sign in with this email to listen.                 │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      🔵 Continue with Google               │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Not sarah@example.com?                             │
│  Ask the sender to share with your email.           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Recipient Opens Link — Wrong Email

```
┌─────────────────────────────────────────────────────┐
│                     Disona                          │
├─────────────────────────────────────────────────────┤
│                                                     │
│              ⚠️ Access Denied                       │
│                                                     │
│  This content was shared with:                      │
│  sarah@example.com                                  │
│                                                     │
│  You're signed in as:                               │
│  mike@example.com                                   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      Switch Account                         │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      Request Access                         │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  "Request Access" will notify the owner to share   │
│  with mike@example.com instead.                    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Recipient Opens Link — Correct Email

```
┌─────────────────────────────────────────────────────┐
│  ←  Deep Work (Shared)                     ⋮       │
├─────────────────────────────────────────────────────┤
│                                                     │
│              📘 Deep Work                           │
│                                                     │
│         Shared by John                              │
│         12 chapters • 4h 30m                        │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  💬 "Hey Sarah! Check out this book about focused  │
│      work. Chapter 4 is especially relevant."      │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           ▶ Play                            │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [⚡ Blitz]  [📖 Full]                 📥 Download  │
│                                                     │
│  Chapters                                           │
│  1. Introduction                          15 min   │
│  2. Deep Work Is Valuable                 28 min   │
│  ...                                               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Managing Shares

### Owner View — Shared Projects

In Project Detail menu or dedicated section:

```
┌─────────────────────────────────────────────────────┐
│  Sharing                                     ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📘 Deep Work                                       │
│                                                     │
│  Shared with:                                       │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  sarah@example.com                          │   │
│  │  Entire project • Shared Apr 10             │   │
│  │  Accessed: Apr 11 (Ch 3)            🗑️      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  mike@example.com                           │   │
│  │  Chapter 4 only • Shared Apr 12             │   │
│  │  Not yet accessed                   🗑️      │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      + Share with Someone Else              │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Revoke Share Confirmation

```
┌─────────────────────────────────────────────────────┐
│  Revoke Access?                              ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Remove sarah@example.com's access to              │
│  "Deep Work"?                                      │
│                                                     │
│  They will no longer be able to listen to this     │
│  content. Any downloads will stop working.         │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Revoke Access                     │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│                    Cancel                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Recipient View — Shared With Me

In Library, "Shared With Me" section:

```
┌─────────────────────────────────────────────────────┐
│  Shared With Me                              (3)   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  📘 Deep Work                               │   │
│  │     from John • Apr 10                      │   │
│  │     Ch 3 of 12 • 25%                        │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  📕 Thinking Fast and Slow                  │   │
│  │     from Sarah • Apr 8                      │   │
│  │     Chapter 2 only                          │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  📗 The Mom Test                            │   │
│  │     from Alex • Apr 5                       │   │
│  │     Clip • 2:34                             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Share Link Expiration

### Default: No Expiration

Shares don't expire by default.

### Optional Expiration (Future)

```
┌─────────────────────────────────────────────────────┐
│  Link expires:                                      │
│                                                     │
│  ○ Never                                           │
│  ○ In 7 days                                       │
│  ○ In 30 days                                      │
│  ○ Custom date: [________]                         │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Expired Link

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│              ⏰ Link Expired                        │
│                                                     │
│  This share link is no longer valid.               │
│                                                     │
│  Contact the person who shared this with you       │
│  to request a new link.                            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Clip Sharing

### Creating a Clip Share

During playback, user can share current position:

```
┌─────────────────────────────────────────────────────┐
│  Share Clip                                  ✕     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📘 Deep Work                                       │
│  Chapter 4: Deep Work Is Meaningful                 │
│                                                     │
│  Clip preview:                                      │
│                                                     │
│  Start: 12:34  ──────●────────  End: 15:08         │
│                                                     │
│  Duration: 2:34                                     │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      ▶ Preview Clip                         │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │           Share This Clip                   │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Clip Recipient View

```
┌─────────────────────────────────────────────────────┐
│  ←  Clip from Deep Work                    ⋮       │
├─────────────────────────────────────────────────────┤
│                                                     │
│              📘 Deep Work                           │
│         Chapter 4: Deep Work Is Meaningful          │
│                                                     │
│         Clip shared by John                         │
│         2:34 duration                               │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│   0:00  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  2:34      │
│                                                     │
│       ◀◀        ▶️        ▶▶                       │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  💬 "Listen to this part about attention residue!" │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Want to hear more?                                 │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │      Request Full Access                    │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## API Interactions

### Create Share

```http
POST /api/shares
Content-Type: application/json

{
  "project_id": "uuid",           // Required for project/chapter share
  "chapter_id": "uuid",           // Optional: specific chapter
  "segment_id": "uuid",           // Optional: specific segment (clip)
  "recipient_email": "sarah@example.com",
  "personal_message": "Check out this book!",
  "expires_at": null              // Optional: ISO timestamp
}
```

**Response:**
```json
{
  "id": "uuid",
  "token": "abc123xyz",
  "share_url": "https://disona.io/s/abc123xyz",
  "recipient_email": "sarah@example.com",
  "share_type": "project",
  "created_at": "2026-04-12T10:30:00Z",
  "expires_at": null
}
```

### Redeem Share (Recipient)

```http
POST /api/shares/{token}/redeem
Authorization: Bearer <jwt>
```

**Response (success):**
```json
{
  "share": {
    "id": "uuid",
    "share_type": "project",
    "sharer": {
      "name": "John",
      "email": "john@example.com"
    },
    "personal_message": "Check out this book!",
    "project": {
      "id": "uuid",
      "title": "Deep Work",
      "chapters_count": 12
    }
  },
  "access_granted": true
}
```

**Response (wrong email):**
```json
{
  "error": "email_mismatch",
  "expected_email": "sarah@example.com",
  "your_email": "mike@example.com"
}
```

### List My Shares (Owner)

```http
GET /api/shares?type=sent
```

### List Shared With Me (Recipient)

```http
GET /api/shares?type=received
```

### Revoke Share

```http
DELETE /api/shares/{id}
```

---

## Notifications

### Email to Recipient

Sent immediately when share is created.

### Push Notification (If App Installed)

```
┌─────────────────────────────────────────────────────┐
│  Disona                                    now     │
│  John shared "Deep Work" with you                  │
│  Tap to listen                                     │
└─────────────────────────────────────────────────────┘
```

### Access Request Notification (To Owner)

When someone requests access with wrong email:

```
┌─────────────────────────────────────────────────────┐
│  Disona                                    now     │
│  mike@example.com wants access to "Deep Work"     │
│  Tap to share with them                            │
└─────────────────────────────────────────────────────┘
```

---

## Security Considerations

### Email Verification

- Share links are tied to specific email addresses
- Recipient must authenticate via OAuth with matching email
- No way to bypass email check

### Token Security

- Share tokens are 20+ character random strings
- Not guessable or enumerable
- Each share has unique token

### Revocation

- Owner can revoke at any time
- Revocation is immediate
- Downloaded content may still play offline (acceptable for MVP)

---

## Offline Behavior

### Downloaded Shared Content

- Recipient can download shared content for offline
- Downloads continue working until share is revoked
- After revocation: existing downloads may work, no new downloads

### Sharing While Offline

- "Share" action queued until online
- Show: "Share will be sent when you're back online"

---

## Accessibility

| Element | A11y |
|---------|------|
| Share modal | `role="dialog"` with `aria-modal="true"` |
| Email input | `aria-describedby` explaining email-lock |
| Share type radio | `role="radiogroup"` |
| Copy link button | Announces "Link copied" |
| Recipient list | `role="list"` |
| Revoke button | `aria-label="Revoke access for [email]"` |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Share completion rate | >80% (start → send) |
| Share redemption rate | >60% (sent → opened) |
| Share-to-play rate | >40% (opened → played) |
| Clip share engagement | >30% request full access |

---

## Open Questions

1. **Batch sharing:** Share with multiple emails at once?
2. **Public links:** Allow non-email-locked shares (public)?
3. **Share limits:** Cap shares per project?
4. **Re-share:** Can recipients share with others?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial specification |
