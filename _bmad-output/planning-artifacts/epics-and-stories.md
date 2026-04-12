# Disona — Epic & Story Breakdown

**Version:** 1.0  
**Date:** 2026-04-12  
**Status:** Draft

---

## Overview

This document decomposes the Disona PRD, Architecture, and UX Design requirements into implementable epics and stories for MVP development.

**Total:** 9 Epics, 53 Stories

---

## Requirements Traceability

### Functional Requirements Coverage

| FR Range | Category | Epic |
|----------|----------|------|
| FR1-FR7 | Content Ingestion | E2 |
| FR8-FR15 | Audio Generation | E3 |
| FR16-FR26 | Audio Playback | E4, E5 |
| FR27-FR35 | Library & Organization | E6 |
| FR36-FR41 | Sharing | E7 |
| FR42-FR46 | User Accounts | E1 |
| FR47-FR54 | Subscription & Billing | E8 |
| FR55-FR58 | Onboarding | E1 |
| FR59-FR63 | Admin & Compliance | E9 |
| FR64-FR68 | Error Handling | E9 |

### Non-Functional Requirements Coverage

| NFR Category | Primary Epics |
|--------------|---------------|
| Performance (P1-P8) | E3, E4 |
| Security (S1-S8) | E1, E9 |
| Scalability (SC1-SC5) | E3 |
| Reliability (R1-R6) | E4, E6 |
| Accessibility (A1-A4) | All |

---

## Epic List Summary

| ID | Epic Name | Stories | Priority |
|----|-----------|---------|----------|
| E1 | Authentication & Onboarding | 6 | P0 |
| E2 | Content Upload & Processing | 5 | P0 |
| E3 | Audio Generation Pipeline | 6 | P0 |
| E4 | Audio Player | 8 | P0 |
| E5 | Blitz & Full Mode | 5 | P0 |
| E6 | Library & Organization | 7 | P1 |
| E7 | Sharing | 5 | P1 |
| E8 | Subscription & Billing | 6 | P1 |
| E9 | Admin & Compliance | 5 | P2 |

---

## Epic 1: Authentication & Onboarding

**Goal:** Enable users to sign up, sign in, and experience the product value quickly through guided onboarding and sample content.

**FRs Covered:** FR42-FR46, FR55-FR58

### Story 1.1: Google OAuth Sign Up

As a **new user**,  
I want to **sign up using my Google account**,  
So that I can **start using Disona without creating a new password**.

**Acceptance Criteria:**

- **Given** I am on the landing page
- **When** I tap "Continue with Google"
- **Then** I am redirected to Google OAuth consent screen
- **And** after granting consent, I am returned to Disona as a logged-in user
- **And** my account is created with my Google email and profile picture

**FRs:** FR42, FR43

---

### Story 1.2: Sign In & Sign Out

As a **returning user**,  
I want to **sign in and sign out of my account**,  
So that I can **access my content securely across sessions**.

**Acceptance Criteria:**

- **Given** I have an existing account
- **When** I tap "Sign In" and authenticate with Google
- **Then** I am logged in and see my library
- **And** my session persists for 30 days of inactivity
- **Given** I am logged in
- **When** I tap "Sign Out" in the account menu
- **Then** I am logged out and returned to the landing page

**FRs:** FR43, FR46

---

### Story 1.3: Cross-Device Session

As a **user with multiple devices**,  
I want to **stay logged in across my phone and computer**,  
So that I can **seamlessly switch between devices**.

**Acceptance Criteria:**

- **Given** I am logged in on my phone
- **When** I open Disona on my computer and sign in
- **Then** I see the same account and library on both devices
- **And** my playback position syncs between devices within 5 seconds

**FRs:** FR46, NFR-R4

---

### Story 1.4: Sample Content for New Users

As a **new user**,  
I want to **try sample content without uploading anything**,  
So that I can **understand how Disona works before committing**.

**Acceptance Criteria:**

- **Given** I just signed up
- **When** I land on the empty library
- **Then** I see an option to "Try a Sample"
- **And** tapping it loads a pre-generated sample (e.g., TED talk)
- **And** I can try all three modes (Audiobook, Blitz, Podcast) on the sample

**FRs:** FR55, FR56

---

### Story 1.5: Guided First Upload

As a **new user**,  
I want to **be guided through my first upload**,  
So that I can **successfully create my first audiobook**.

**Acceptance Criteria:**

- **Given** I have completed the sample or skipped it
- **When** I tap "Upload your first PDF"
- **Then** I see a step-by-step guide with tips
- **And** after upload, I see processing progress with encouraging messages
- **And** on completion, I'm prompted to play or generate audio

**FRs:** FR57

---

### Story 1.6: Push Notification Permission

As a **user**,  
I want to **be prompted for push notification permission**,  
So that I can **receive "audio ready" alerts**.

**Acceptance Criteria:**

- **Given** I have uploaded my first content
- **When** processing begins
- **Then** I am prompted to enable push notifications
- **And** the prompt explains the benefit ("We'll notify you when your audio is ready")
- **And** I can dismiss or enable notifications

**FRs:** FR58, FR13

---

## Epic 2: Content Upload & Processing

**Goal:** Allow users to upload PDF documents and have them automatically parsed into a hierarchical chapter structure.

**FRs Covered:** FR1-FR7

### Story 2.1: PDF Upload with Progress

As a **user**,
I want to **upload a PDF file and see upload progress**,
So that I can **know my file is being transferred successfully**.

**Acceptance Criteria:**

- **Given** I tap "Upload a file"
- **When** I select a PDF from my device
- **Then** I see a progress bar showing upload percentage
- **And** I can cancel the upload at any time
- **And** files up to 100MB are accepted

**FRs:** FR1, FR2

---

### Story 2.2: Document Processing & Chapter Detection

As a **user**,
I want to **have my PDF automatically parsed into chapters**,
So that I can **navigate the content by chapter**.

**Acceptance Criteria:**

- **Given** my PDF is uploaded
- **When** processing begins
- **Then** I see status updates: "Parsing PDF", "Detecting chapters", "Analyzing content"
- **And** chapters are detected from PDF structure/headings
- **And** if no chapters detected, content is treated as single chapter
- **And** processing completes within 2 minutes for typical documents

**FRs:** FR4, FR5, NFR-P3

---

### Story 2.3: Auto-Populate Project Metadata

As a **user**,
I want to **have project title and author auto-extracted**,
So that I can **skip manual data entry**.

**Acceptance Criteria:**

- **Given** my PDF is being processed
- **When** metadata extraction runs
- **Then** title is extracted from PDF metadata or filename
- **And** author is extracted from PDF metadata if available
- **And** I can edit title/author later if needed

**FRs:** FR3

---

### Story 2.4: Processing Status Polling

As a **user**,
I want to **see real-time processing status updates**,
So that I can **know what's happening with my document**.

**Acceptance Criteria:**

- **Given** my document is processing
- **When** I am on the library or project detail screen
- **Then** status updates every 3 seconds automatically
- **And** status shows substeps: parsing, detecting, analyzing, summarizing
- **And** on completion, I'm taken to the project detail view

**FRs:** FR6, UX-Upload

---

### Story 2.5: Multi-Document Project (Future)

As a **user**,
I want to **add multiple PDFs to a single project**,
So that I can **combine related documents into one audiobook**.

**Acceptance Criteria:**

- **Given** I have uploaded one PDF and processing is ongoing
- **When** I tap "Add another file"
- **Then** I can select additional PDFs
- **And** chapters from all documents are combined in order
- **And** once processing succeeds, I cannot add more files

**FRs:** FR7, UX-Upload

---

## Epic 3: Audio Generation Pipeline

**Goal:** Generate high-quality TTS audio from document content, including chapter summaries (L1) and key point summaries (L2).

**FRs Covered:** FR8-FR15

### Story 3.1: Trigger Audio Generation

As a **user**,
I want to **trigger audio generation for my project**,
So that I can **listen to my document as an audiobook**.

**Acceptance Criteria:**

- **Given** my project has finished processing
- **When** I tap "Generate Audio"
- **Then** I see options: all chapters or selected chapters
- **And** I see estimated quota usage before confirming
- **And** generation begins after confirmation

**FRs:** FR8, FR9

---

### Story 3.2: Voice Selection

As a **user**,
I want to **choose from multiple TTS voices**,
So that I can **pick a voice I enjoy listening to**.

**Acceptance Criteria:**

- **Given** I am generating audio
- **When** I tap "Change Voice"
- **Then** I see a list of available voices with language and gender
- **And** I can preview each voice with a sample
- **And** my selection is saved for this project

**FRs:** FR11

---

### Story 3.3: Generation Progress Tracking

As a **user**,
I want to **see audio generation progress per chapter**,
So that I can **know when each chapter will be ready**.

**Acceptance Criteria:**

- **Given** audio is generating
- **When** I view the project detail
- **Then** I see overall progress (e.g., "Chapter 3 of 12")
- **And** each chapter shows: queued, generating, or ready
- **And** I can start listening to completed chapters immediately

**FRs:** FR12

---

### Story 3.4: Generation Completion Notification

As a **user**,
I want to **be notified when audio generation is complete**,
So that I can **start listening without checking repeatedly**.

**Acceptance Criteria:**

- **Given** I have enabled push notifications
- **When** audio generation completes
- **Then** I receive a push notification: "Deep Work is ready to listen"
- **And** tapping the notification opens the project

**FRs:** FR13

---

### Story 3.5: L1/L2 Summary Generation

As a **user**,
I want to **have chapter and key point summaries generated**,
So that I can **use Blitz mode for quick overview**.

**Acceptance Criteria:**

- **Given** my project is being processed
- **When** the LLM worker processes each chapter
- **Then** L1 (chapter summary ~1 min) is generated
- **And** L2 (key point summaries ~45s each) are generated
- **And** summaries are stored and linked to corresponding positions in full chapter

**FRs:** FR9, FR10, Architecture-LLM

---

### Story 3.6: Quota Enforcement

As a **user**,
I want to **see my quota usage before generating**,
So that I can **avoid overages or upgrade my plan**.

**Acceptance Criteria:**

- **Given** I tap "Generate Audio"
- **When** estimated time exceeds my remaining quota
- **Then** I see a warning with upgrade option
- **And** I can choose to generate only what my quota allows
- **And** quota is deducted only for successfully generated audio

**FRs:** FR14, FR47

---

## Epic 4: Audio Player

**Goal:** Provide a full-featured audio player with mini player, full player, transport controls, and background/lock screen playback.

**FRs Covered:** FR16-FR26

### Story 4.1: Mini Player

As a **user**,
I want to **see a persistent mini player when audio is playing**,
So that I can **control playback from any screen**.

**Acceptance Criteria:**

- **Given** audio is playing
- **When** I navigate away from the player
- **Then** a mini player appears at the bottom of the screen
- **And** mini player shows: cover, title, chapter, play/pause, +15s
- **And** tapping the mini player expands to full player

**FRs:** FR16, UX-Player

---

### Story 4.2: Full Player Expansion

As a **user**,
I want to **expand the mini player to full player**,
So that I can **access all playback controls**.

**Acceptance Criteria:**

- **Given** the mini player is visible
- **When** I tap or swipe up on the mini player
- **Then** it smoothly expands to full player (300ms animation)
- **And** full player shows: large cover, 5-button transport, progress bar, speed, mode toggle
- **And** I can collapse back to mini player by swiping down or tapping collapse button

**FRs:** FR17, UX-Player

---

### Story 4.3: Transport Controls

As a **user**,
I want to **use play, pause, skip, and seek controls**,
So that I can **navigate through the audio**.

**Acceptance Criteria:**

- **Given** I am in the player
- **When** I use transport controls
- **Then** play/pause toggles playback
- **And** +15s/-15s skips forward/backward
- **And** prev/next moves between segments (Blitz) or chapters (Full)
- **And** I can drag the progress bar to seek

**FRs:** FR18, FR19, FR20

---

### Story 4.4: Playback Speed Control

As a **user**,
I want to **adjust playback speed**,
So that I can **listen faster or slower based on content**.

**Acceptance Criteria:**

- **Given** I am in the full player
- **When** I tap the speed button
- **Then** I see speed options: 0.5x, 0.75x, 1x, 1.25x, 1.5x, 1.75x, 2x, 2.5x, 3x
- **And** selecting a speed applies immediately
- **And** my speed preference is saved per user (not per project)

**FRs:** FR21

---

### Story 4.5: Background & Lock Screen Playback

As a **user**,
I want to **continue listening when the app is in the background**,
So that I can **multitask while listening**.

**Acceptance Criteria:**

- **Given** audio is playing
- **When** I switch apps or lock my phone
- **Then** audio continues playing
- **And** lock screen shows media controls with title, chapter, play/pause, skip
- **And** notification shows playback controls

**FRs:** FR22, FR23, UX-Player

---

### Story 4.6: Progress Persistence

As a **user**,
I want to **have my playback position saved automatically**,
So that I can **resume where I left off**.

**Acceptance Criteria:**

- **Given** I am listening to audio
- **When** I pause, close the app, or switch devices
- **Then** my position is saved (within 10 seconds of last update)
- **And** when I return, playback resumes at the saved position
- **And** position syncs across devices

**FRs:** FR24, NFR-R4

---

### Story 4.7: Chapter Navigation

As a **user**,
I want to **browse and jump to specific chapters**,
So that I can **find content I'm interested in**.

**Acceptance Criteria:**

- **Given** I am in the full player
- **When** I tap the chapter browser
- **Then** I see all chapters with status (completed, in progress, not started)
- **And** tapping a chapter jumps to that chapter
- **And** current chapter is highlighted

**FRs:** FR25, UX-Player

---

### Story 4.8: Download for Offline

As a **user**,
I want to **download audio for offline listening**,
So that I can **listen without internet connection**.

**Acceptance Criteria:**

- **Given** a project has generated audio
- **When** I tap "Download"
- **Then** I see download progress and estimated size
- **And** once downloaded, I can listen offline
- **And** downloaded projects show a "Downloaded" badge

**FRs:** FR26, NFR-R5

---

## Epic 5: Blitz & Full Mode

**Goal:** Enable users to choose between quick summary listening (Blitz) and full narration (Full), with seamless "Go Deep" transitions.

**FRs Covered:** FR9, FR10, UX-Blitz/Full

### Story 5.1: Mode Toggle

As a **user**,
I want to **switch between Blitz and Full mode**,
So that I can **control how much depth I get**.

**Acceptance Criteria:**

- **Given** I am playing a project
- **When** I tap the mode toggle
- **Then** I can switch between Blitz ⚡ and Full 📖
- **And** my position maps correctly between modes
- **And** visual styling indicates active mode (amber for Blitz, blue for Full)

**FRs:** FR10, UX-Blitz/Full

---

### Story 5.2: Blitz Mode Playback

As a **user**,
I want to **listen to chapter and key point summaries in Blitz mode**,
So that I can **quickly grasp the main points**.

**Acceptance Criteria:**

- **Given** I am in Blitz mode
- **When** a chapter plays
- **Then** I hear L1 (chapter summary ~1 min) first
- **And** then L2 (key point summaries ~45s each) in sequence
- **And** navigation skips between L2 key points

**FRs:** FR9, UX-Blitz/Full

---

### Story 5.3: Go Deep Transition

As a **user**,
I want to **tap "Go Deep" during a key point**,
So that I can **hear the full chapter from that point**.

**Acceptance Criteria:**

- **Given** I am listening to an L2 key point in Blitz mode
- **When** I tap "Go Deep"
- **Then** playback switches to Full mode
- **And** position jumps to the corresponding timestamp in full chapter
- **And** after chapter ends, I'm asked to return to Blitz or stay in Full

**FRs:** UX-Blitz/Full

---

### Story 5.4: Return to Blitz

As a **user**,
I want to **return to Blitz mode after going deep**,
So that I can **continue quickly through remaining chapters**.

**Acceptance Criteria:**

- **Given** I used "Go Deep" and the chapter ended
- **When** auto-advance triggers
- **Then** I'm prompted: "Return to Blitz Mode?" with checkbox "Stay in Full Mode"
- **And** if unchecked, next chapter plays in Blitz (L1 summary)
- **And** if checked, next chapter plays in Full

**FRs:** UX-Blitz/Full

---

### Story 5.5: Key Point Markers in Full Mode

As a **user**,
I want to **see key point markers on the progress bar in Full mode**,
So that I can **jump to specific sections**.

**Acceptance Criteria:**

- **Given** I am in Full mode
- **When** I view the progress bar
- **Then** I see tick marks at each key point position
- **And** tapping a marker jumps to that position
- **And** the current section shows the key point title

**FRs:** UX-Blitz/Full

---

## Epic 6: Library & Organization

**Goal:** Provide a well-organized library view with filtering, sorting, project management, and offline support.

**FRs Covered:** FR27-FR35

### Story 6.1: Library Home Screen

As a **user**,
I want to **see my projects on the library screen after login**,
So that I can **access my content easily**.

**Acceptance Criteria:**

- **Given** I am logged in
- **When** I land on the home screen
- **Then** I see my library with projects sorted by last played
- **And** "Continue Listening" section shows most recent active project
- **And** "Upload a file" button is visible

**FRs:** FR27, UX-Library

---

### Story 6.2: Filter Tabs

As a **user**,
I want to **filter my library by status**,
So that I can **find projects in specific states**.

**Acceptance Criteria:**

- **Given** I am on the library screen
- **When** I tap filter tabs
- **Then** "All" shows all projects
- **And** "In Progress" shows projects with 0% < progress < 100%
- **And** "Completed" shows projects with 100% progress

**FRs:** FR28, UX-Library

---

### Story 6.3: Project Card States

As a **user**,
I want to **see project status at a glance**,
So that I can **know what's ready and what's processing**.

**Acceptance Criteria:**

- **Given** I am viewing the library
- **When** I look at project cards
- **Then** each card shows: title, progress bar, status indicator
- **And** processing projects show processing status
- **And** ready projects show chapter count and duration

**FRs:** FR29, UX-Library

---

### Story 6.4: Pull to Refresh

As a **user**,
I want to **refresh my library by pulling down**,
So that I can **sync the latest status from server**.

**Acceptance Criteria:**

- **Given** I am on the library screen
- **When** I pull down
- **Then** library refreshes and syncs with server
- **And** processing statuses update
- **And** playback progress syncs across devices

**FRs:** FR30

---

### Story 6.5: Project Context Menu

As a **user**,
I want to **access project actions via long press**,
So that I can **manage projects without opening them**.

**Acceptance Criteria:**

- **Given** I am on the library screen
- **When** I long press a project
- **Then** I see context menu: Play, Download, Share, Rename, Delete
- **And** tapping an option performs that action

**FRs:** FR31, FR32, FR33

---

### Story 6.6: Delete Project

As a **user**,
I want to **delete projects I no longer need**,
So that I can **keep my library clean**.

**Acceptance Criteria:**

- **Given** I tap Delete on a project
- **When** the confirmation dialog appears
- **Then** I see warning about permanent deletion
- **And** if project is shared, I see share impact warning
- **And** confirming deletes the project and all audio

**FRs:** FR34

---

### Story 6.7: Empty Library State

As a **new user**,
I want to **see helpful guidance when my library is empty**,
So that I can **understand how to get started**.

**Acceptance Criteria:**

- **Given** I have no projects
- **When** I view the library
- **Then** I see an empty state with helpful message
- **And** "Upload a file" button is prominent
- **And** "Try a Sample" option is visible

**FRs:** FR35, UX-Library

---

## Epic 7: Sharing

**Goal:** Allow users to share projects, chapters, or clips with others via email-locked links.

**FRs Covered:** FR36-FR41

### Story 7.1: Share Project

As a **user**,
I want to **share a project with someone via email**,
So that they can **listen to content I found valuable**.

**Acceptance Criteria:**

- **Given** I have a project with generated audio
- **When** I tap Share and enter an email
- **Then** a share link is created locked to that email
- **And** recipient receives an email notification
- **And** I see confirmation with the share link

**FRs:** FR36, FR37, UX-Share

---

### Story 7.2: Share Chapter or Clip

As a **user**,
I want to **share a specific chapter or clip**,
So that I can **highlight a particular section**.

**Acceptance Criteria:**

- **Given** I am playing or viewing a project
- **When** I tap Share and select "Chapter" or "Clip"
- **Then** I can share just that chapter or a time range
- **And** recipient can only access the shared portion
- **And** "Request Full Access" option is shown to recipient

**FRs:** FR38, UX-Share

---

### Story 7.3: Recipient Authentication

As a **share recipient**,
I want to **access shared content with my email**,
So that I can **listen to what was shared with me**.

**Acceptance Criteria:**

- **Given** I received a share link
- **When** I open the link
- **Then** I'm prompted to sign in with the specified email
- **And** after authentication, I gain access to the content
- **And** if I use wrong email, I see "Access Denied" with "Request Access" option

**FRs:** FR39, UX-Share

---

### Story 7.4: Manage Shares (Owner)

As a **project owner**,
I want to **see who I've shared with and revoke access**,
So that I can **control who has access to my content**.

**Acceptance Criteria:**

- **Given** I have shared a project
- **When** I open the sharing management view
- **Then** I see list of recipients with access status
- **And** I can revoke access for any recipient
- **And** revoked recipients immediately lose access

**FRs:** FR40, UX-Share

---

### Story 7.5: Shared With Me Section

As a **share recipient**,
I want to **see content shared with me in my library**,
So that I can **easily find and listen to shared content**.

**Acceptance Criteria:**

- **Given** someone shared content with me
- **When** I view my library
- **Then** I see a "Shared With Me" section
- **And** each item shows who shared it and when
- **And** tapping opens the shared content

**FRs:** FR41, UX-Share

---

## Epic 8: Subscription & Billing

**Goal:** Implement subscription tiers with quota-based usage limits and Stripe payment integration.

**FRs Covered:** FR47-FR54

### Story 8.1: View Subscription Status

As a **user**,
I want to **see my current plan and usage**,
So that I can **understand my quota limits**.

**Acceptance Criteria:**

- **Given** I am logged in
- **When** I tap my avatar and view account menu
- **Then** I see my current plan (Free, Standard, Pro)
- **And** I see usage: "Upload: 12h / 50h", "Audio: 8h / 20h"
- **And** I see usage percentage bars

**FRs:** FR47, FR48

---

### Story 8.2: Quota Warning

As a **user**,
I want to **be warned when approaching quota limits**,
So that I can **decide to upgrade before hitting limits**.

**Acceptance Criteria:**

- **Given** I am at 80% of my quota
- **When** I trigger audio generation
- **Then** I see a warning: "You're running low on quota"
- **And** I see option to upgrade or continue
- **And** at 100%, generation is blocked with upgrade prompt

**FRs:** FR49, FR50

---

### Story 8.3: Upgrade Flow

As a **user**,
I want to **upgrade my subscription plan**,
So that I can **get more quota**.

**Acceptance Criteria:**

- **Given** I want to upgrade
- **When** I tap "Upgrade" or "View Plans"
- **Then** I see plan comparison: Free, Standard ($9/mo), Pro ($19/mo)
- **And** I can select a plan and enter payment via Stripe
- **And** after payment, my quota is immediately increased

**FRs:** FR51, FR52

---

### Story 8.4: Payment Management

As a **paying user**,
I want to **manage my payment method and view history**,
So that I can **update my card or check invoices**.

**Acceptance Criteria:**

- **Given** I am a paying subscriber
- **When** I go to Subscription settings
- **Then** I see my current card (last 4 digits)
- **And** I can update payment method
- **And** I can view billing history and download invoices

**FRs:** FR52, FR53

---

### Story 8.5: Cancel Subscription

As a **paying user**,
I want to **cancel my subscription**,
So that I can **stop being charged**.

**Acceptance Criteria:**

- **Given** I am a paying subscriber
- **When** I tap "Cancel Subscription"
- **Then** I see what I'll lose (reduced quota)
- **And** I'm asked for cancellation reason (optional)
- **And** after cancellation, I keep access until period ends

**FRs:** FR54

---

### Story 8.6: Free Tier Limits

As a **free user**,
I want to **use Disona with limited quota**,
So that I can **try the product before paying**.

**Acceptance Criteria:**

- **Given** I am on the free plan
- **When** I use the app
- **Then** I have upload quota: 5h/month, audio quota: 2h/month
- **And** I see gentle upgrade prompts when useful
- **And** I can use all features, just with lower limits

**FRs:** FR47

---

## Epic 9: Admin & Compliance

**Goal:** Implement admin tools for monitoring, moderation, and compliance with data protection regulations.

**FRs Covered:** FR59-FR68

### Story 9.1: Usage Analytics Dashboard

As an **admin**,
I want to **view usage analytics**,
So that I can **monitor system health and user activity**.

**Acceptance Criteria:**

- **Given** I am an admin
- **When** I access the admin dashboard
- **Then** I see: active users, uploads today, audio generated, error rates
- **And** I can filter by time period
- **And** I see charts for trends

**FRs:** FR59

---

### Story 9.2: Content Moderation

As an **admin**,
I want to **review flagged content**,
So that I can **remove inappropriate material**.

**Acceptance Criteria:**

- **Given** content is flagged (automated or reported)
- **When** I view the moderation queue
- **Then** I see flagged projects with reason
- **And** I can review content and take action: approve, remove, warn user

**FRs:** FR60

---

### Story 9.3: User Data Export (GDPR)

As a **user**,
I want to **export my data**,
So that I can **comply with my rights under GDPR**.

**Acceptance Criteria:**

- **Given** I am logged in
- **When** I request data export in settings
- **Then** I receive a download link within 24 hours
- **And** export includes: account info, projects, playback history, shares

**FRs:** FR61, NFR-S7

---

### Story 9.4: Account Deletion (Right to be Forgotten)

As a **user**,
I want to **delete my account and all data**,
So that I can **exercise my right to be forgotten**.

**Acceptance Criteria:**

- **Given** I want to delete my account
- **When** I tap "Delete Account" and confirm
- **Then** all my data is permanently deleted within 30 days
- **And** I receive confirmation email
- **And** shared content becomes inaccessible to recipients

**FRs:** FR62, NFR-S7

---

### Story 9.5: Error Handling & Support

As a **user**,
I want to **receive helpful error messages and access support**,
So that I can **resolve issues quickly**.

**Acceptance Criteria:**

- **Given** an error occurs (upload fail, processing error, playback issue)
- **When** I see the error
- **Then** I see a clear, actionable error message
- **And** I see retry option where applicable
- **And** I can access support via help menu

**FRs:** FR64, FR65, FR66, FR67, FR68

---

## Appendix A: Story Sizing Reference

| Size | Story Points | Time Estimate |
|------|--------------|---------------|
| XS | 1 | < 0.5 day |
| S | 2 | 0.5 - 1 day |
| M | 3-5 | 1 - 3 days |
| L | 8 | 3 - 5 days |
| XL | 13+ | > 5 days (should be split) |

---

## Appendix B: Priority Definitions

| Priority | Meaning |
|----------|---------|
| P0 | Must have for MVP launch |
| P1 | Should have for MVP, could defer |
| P2 | Nice to have, post-MVP |

---

## Appendix C: Definition of Done

A story is "Done" when:

1. ✅ Code is written and follows project conventions
2. ✅ Unit tests pass with >80% coverage
3. ✅ Integration tests pass
4. ✅ Code reviewed and approved
5. ✅ Acceptance criteria verified
6. ✅ Documentation updated (if applicable)
7. ✅ Deployed to staging and tested
8. ✅ No critical bugs

---

## Changelog

| Date | Change |
|------|--------|
| 2026-04-12 | Initial epic and story breakdown |
