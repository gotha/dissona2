# User Journeys

## Overview

This document describes the primary user journeys through Disona, from first visit to regular usage.

---

## Journey 1: First-Time User — Discovery to First Listen

### Persona: Alex, busy professional

**Goal:** Discover Disona, understand the value, try it out

### Flow

```
1. Landing Page
   │
   ├── Sees value proposition: "Turn books into podcasts"
   ├── Sees sample content available
   │
   ▼
2. Browse Samples (No Account Required)
   │
   ├── Clicks "Try a Sample"
   ├── Sees 3-5 sample books
   ├── Selects "The Manager's Path" sample
   │
   ▼
3. Sample Playback Experience
   │
   ├── Lands on Chapter 1 in Blitz Mode (L1 summary)
   ├── Listens to 2-minute overview
   ├── Taps "Go Deep" on interesting point
   ├── Hears detailed explanation
   ├── Impressed by seamless transition
   │
   ▼
4. Conversion Gate
   │
   ├── Tries to access Chapter 2 or upload own book
   ├── Sees "Sign up to continue" modal
   ├── Options: Google OAuth or Email
   │
   ▼
5. Sign Up
   │
   ├── Clicks "Continue with Google"
   ├── OAuth flow completes
   ├── Lands on empty library with "Upload a book" CTA
   │
   ▼
6. First Upload
   │
   ├── Clicks "Upload"
   ├── Selects PDF from device
   ├── Sees upload progress
   ├── Sees "Analyzing your book..." status
```

### Services Involved

| Step | Service | Action |
|------|---------|--------|
| 1-3 | API Service | Serve sample content |
| 4-5 | Auth Service | Google OAuth |
| 6 | API Service | Upload PDF, create book record |
| 6 | PDF Worker | Parse, extract, detect chapters |
| 6 | LLM Worker | Generate summaries |

---

## Journey 2: Upload and Generate — PDF to Audio

### Persona: Jordan, learning enthusiast

**Goal:** Upload a PDF and start listening

### Flow

```
1. Library View (Authenticated)
   │
   ├── Clicks "Upload" button
   │
   ▼
2. File Selection
   │
   ├── Native file picker opens
   ├── Selects "Atomic Habits.pdf"
   ├── File uploads (progress indicator)
   │
   ▼
3. Analysis Phase (Automatic)
   │
   ├── Status: "Analyzing your book..."
   ├── Progress: "Extracting text..."
   ├── Progress: "Detecting chapters... (12 found)"
   ├── Progress: "Generating summaries..."
   ├── Status changes to: "Ready to generate audio"
   │
   ▼
4. Generate Audio (User-Initiated)
   │
   ├── Sees book card with "Generate Audio" button
   ├── Sees estimated duration: "~4 hours"
   ├── Sees quota: "You have 10 hours remaining"
   ├── Clicks "Generate Audio"
   │
   ▼
5. Generation Phase
   │
   ├── Status: "Generating audio..."
   ├── Progress: "Chapter 1 ready" (can start listening!)
   ├── Progress: "Chapter 2 ready"
   ├── ... (continues in background)
   │
   ▼
6. First Listen
   │
   ├── Taps Chapter 1 (while others still generating)
   ├── Enters Blitz Mode by default
   ├── L1 summary starts playing
```

### Key Behaviors

| Behavior | Description |
|----------|-------------|
| **Progressive availability** | User can listen to Chapter 1 while Chapter 5 is generating |
| **Two-stage processing** | Analysis is automatic; audio generation is user-triggered |
| **Quota visibility** | User sees remaining hours before committing |

---

## Journey 3: Listening — Blitz Mode to Go Deep

### Persona: Sam, commuter

**Goal:** Learn key concepts during commute, dive deeper on interesting topics

### Flow

```
1. Open App (Commute Starting)
   │
   ├── Sees "Continue Listening" with last book
   ├── Taps to resume
   │
   ▼
2. Blitz Mode (Default)
   │
   ├── L1 summary plays (chapter overview)
   ├── Hears: "This chapter covers three key strategies..."
   │
   ▼
3. Key Points (L2 Summaries)
   │
   ├── First key point summary plays
   ├── "Key Point 1: The 2-Minute Rule..."
   ├── Sam thinks: "I want to know more about this"
   │
   ▼
4. Go Deep (User-Initiated)
   │
   ├── Taps "Go Deep" button
   ├── Audio seamlessly transitions to full chapter
   ├── Starts exactly where this key point begins
   ├── "Temporary Deep" checkbox appears
   │
   ▼
5. Deep Listening
   │
   ├── Listens to full explanation (~5 minutes)
   ├── "Temporary Deep" was ON
   ├── After this section, auto-returns to Blitz
   │
   ▼
6. Return to Blitz
   │
   ├── Audio transitions back to next L2 summary
   ├── "Key Point 2: Habit Stacking..."
   ├── Sam continues in summary mode
```

### Go Deep Modes

| Mode | Behavior |
|------|----------|
| **Temporary Deep** | Listen to this section in full, then return to Blitz |
| **Full Deep** | Switch to Full Mode for rest of chapter |

---

## Journey 4: Hands-Free — Lock Screen Listening

### Persona: Riley, gym-goer

**Goal:** Listen during workout without touching phone

### Flow

```
1. Start Workout
   │
   ├── Opens Disona, starts playing
   ├── Locks phone, puts in pocket
   │
   ▼
2. Lock Screen Controls
   │
   ├── Play/pause visible on lock screen
   ├── Skip forward/back (30 sec)
   ├── Chapter title and book shown
   │
   ▼
3. Background Playback
   │
   ├── Audio continues playing
   ├── Phone call comes in → audio pauses
   ├── Call ends → audio resumes
   │
   ▼
4. Chapter Transition
   │
   ├── Chapter ends
   ├── Brief pause (1 sec)
   ├── Next chapter begins automatically
   ├── Lock screen updates with new chapter title
```

### Technical Requirements

| Requirement | Implementation |
|-------------|----------------|
| Lock screen controls | MediaSession API |
| Background audio | Service Worker + Audio element |
| Phone call handling | Audio focus management |

---

## Journey 5: Offline — Download and Commute

### Persona: Casey, subway commuter

**Goal:** Download book at home, listen offline during commute

### Flow

```
1. At Home (WiFi)
   │
   ├── Opens book in library
   ├── Taps "Download" icon
   │
   ▼
2. Download Progress
   │
   ├── "Downloading... 45%"
   ├── Shows size: "127 MB"
   ├── Completes: "Downloaded ✓"
   │
   ▼
3. Next Morning (Subway, No Signal)
   │
   ├── Opens Disona
   ├── App loads instantly (cached)
   ├── Library shows downloaded book
   │
   ▼
4. Offline Playback
   │
   ├── Taps book, starts playing
   ├── All features work (Blitz, Go Deep)
   ├── Progress tracked locally
   │
   ▼
5. Back Online (Evening)
   │
   ├── App detects connection
   ├── Syncs progress to cloud (background)
   ├── Other devices see updated position
```

### Offline Capabilities

| Feature | Offline Support |
|---------|-----------------|
| Audio playback | ✅ Full |
| Go Deep transition | ✅ Full |
| Progress tracking | ✅ Local, syncs later |
| Library browsing | ✅ Downloaded books |
| New uploads | ❌ Requires connection |

---

## Journey 6: Sharing — Gift a Chapter

### Persona: Morgan, team lead

**Goal:** Share an insightful chapter with team member

### Flow

```
1. Listening to Chapter 5
   │
   ├── "This would be perfect for Jamie"
   ├── Taps share icon
   │
   ▼
2. Share Options
   │
   ├── Sees sharing options:
   │   - Share chapter
   │   - Share specific quote (highlight)
   ├── Selects "Share Chapter"
   │
   ▼
3. Configure Share
   │
   ├── Enter recipient email: "jamie@company.com"
   ├── Add personal message: "Check out this chapter on delegation!"
   ├── Taps "Send"
   │
   ▼
4. Recipient Experience (Jamie)
   │
   ├── Receives email: "Morgan shared a chapter with you"
   ├── Clicks link in email
   │
   ▼
5. Share Landing Page
   │
   ├── Sees: "Morgan shared 'The Art of Delegation' with you"
   ├── Sees personal message
   ├── Enters email to verify (same as invite)
   │
   ▼
6. Access Content
   │
   ├── Email verified
   ├── Chapter appears in "Shared With Me"
   ├── Can listen without subscription
   ├── CTA: "Want to upload your own books? Sign up!"
```

### Sharing Rules

| Aspect | Behavior |
|--------|----------|
| Who can share | Paid subscribers only |
| What can be shared | Chapters, quotes |
| Access control | Email-locked (recipient must verify) |
| Recipient limits | Unlimited listens |
| Conversion | Shared content acts as teaser |
