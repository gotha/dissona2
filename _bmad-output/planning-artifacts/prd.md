---
title: "Product Requirements Document - Disona"
status: final
version: "1.0"
author: Gotha
created: 2026-04-08
updated: 2026-04-08
reviewers: []
classification:
  projectType: web_app
  domain: edtech
  complexity: medium
  projectContext: greenfield
---

# Product Requirements Document - Disona

| Field | Value |
|-------|-------|
| **Author** | Gotha |
| **Status** | Final |
| **Version** | 1.0 |
| **Created** | 2026-04-08 |
| **Last Updated** | 2026-04-08 |

---

## Glossary

| Term | Definition |
|------|------------|
| **Audiobook Mode** | Full narration of uploaded content, chunked into chapter episodes |
| **Blitz Mode** | ~1 minute chapter summaries with key points; user can "go deep" into full chapter |
| **Podcast Mode** | AI-generated multi-host (2-4 voices) discussion exploring ideas from uploaded content |
| **Progressive Generation** | First chapter audio ready within 60 seconds; remaining chapters generate while user listens |
| **Depth on Demand** | Core UX pattern: skim summaries, then dive into full content where it matters |
| **Creator** | User who uploads content and generates audio (paid tier) |
| **Recipient** | User who receives shared content (can be free tier) |
| **PWA** | Progressive Web App — installable web application with offline support |

---

## Changelog

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-08 | Gotha | Initial PRD created via BMAD workflow |

---

## Executive Summary

> **A grad student has 30 papers and a 45-minute commute — Disona turns her reading list into her commute playlist.** We transform books and papers into intelligent audio: skim summaries, go deep on what matters, or hear ideas debated. Your knowledge, your format, your audio to keep and share. €5/month for 20 hours. Grad students first, 2,000 subscribers year one.

---

**Disona** is a mobile-first Progressive Web App that transforms books, research papers, and articles into intelligent audio — enabling users to learn during moments when reading isn't possible.

**The core insight:** Your reading list isn't a time problem — it's a format problem. Knowledge is trapped in text, but available attention exists during commutes, workouts, and chores. Audio isn't better than reading; it's *possible* when reading isn't.

**Your knowledge, your format.** Three consumption modes:
- **Audiobook Mode** — Full narration, intelligently chunked into chapter episodes
- **Blitz Mode** — ~1 minute chapter summaries with depth on demand
- **Podcast Mode** — AI-generated multi-host discussions exploring ideas from multiple angles

**Target users:** Graduate students and knowledge workers with real learning pressure.

**The differentiator:** Creator-first. Upload *your* sources, control *your* depth, share *your* curated audio.

**Business model:** €5/month (20 hrs) | €20/month (100 hrs) | Free tier: listen to shared content only.

**Year 1 target:** 2,000 actively paying subscribers.

### What Makes This Special

1. **Format bridge:** The only solution that makes learning *possible* during hands-busy moments — not just easier.

2. **Depth on demand:** Unlike Blinkist (generic summaries) or Audible (full narration only), Disona lets you skim structure then dive deep on exactly what matters to you.

3. **Creator-first model:** You're not consuming someone else's interpretation. You upload your sources, you control the output, you own the result.

4. **Sharing as gifting:** When you share, you're giving a curated experience you created — not pushing an ad.

5. **Grad student wedge:** Specific, underserved audience with real pressure, high content volume, and vocal communities.

### Project Classification

| Dimension | Value | Notes |
|-----------|-------|-------|
| **Project Type** | Web App (PWA) | Mobile-first, offline-capable, cross-device sync |
| **Domain** | EdTech | Learning/education focus, privacy considerations (GDPR) |
| **Complexity** | Medium | Content handling and quality control matter |
| **Project Context** | Greenfield | New product, no existing codebase |

## Success Criteria

### User Success

**Time-to-Value: Instant Gratification**
- First chapter audio available within 60 seconds of upload completion
- Progressive generation: Listen to Chapter 1 while Chapters 2-N generate
- Pre-parsing: Files in folders chunked automatically before audio request

**Engagement Pattern: Daily Habit**
- Target: 20-30 minutes of listening per day
- Habit triggers: Push notifications, "continue listening" prompts

**The Real Aha Moment**
> "I've listened to 2-3 books this month that I never would have read otherwise."

**Friction Reduction**
- Upload → first audio < 5 minutes total flow
- Push notification when audio ready
- Resume from exact second across devices

### Business Success

| Metric | Target | Timeframe |
|--------|--------|-----------|
| Paying subscribers | 2,000 | Month 12 |
| MRR | €20K+ | Month 12 |
| Free → Paid conversion | 8% | Ongoing |
| Monthly churn | < 7% | Ongoing |
| Shares per creator | 2/month | Ongoing |
| Contribution margin | Positive | Month 6 |
| TTS unit cost | < €0.20/hour | Launch |

### Technical Success

| Metric | Target |
|--------|--------|
| First chapter ready | < 60 seconds |
| Full book (300 pages) | < 10 minutes |
| Push notification delivery | < 30 seconds after ready |
| Cross-device sync | Resume from exact second |
| Audio quality | Latest open-source TTS, natural-sounding |

### Measurable Outcomes

| Outcome | Metric | Target |
|---------|--------|--------|
| Activation | Upload to first play | < 5 min |
| Habit | Daily listening | 20-30 min |
| Value realization | Books/month | 2-3 |
| Quality perception | User rating | > 4.0/5 |
| Growth loop | Shares/creator/month | 2 |

## Product Scope

### MVP — Minimum Viable Product

- PDF upload + automatic chapter detection
- Progressive audio generation (first chapter fast)
- Three modes: Audiobook, Blitz, Podcast
- Resume from exact second
- Push notifications when ready
- Playlist/queue for continuous listening
- Variable playback speed (0.5x - 3x)
- Private sharing (in-app + email invite)
- Two tiers: €5/20hrs, €20/100hrs
- Free tier: listen to shared only
- PWA with offline download
- Folder organization with pre-parsing
- Chapter markers / timestamps
- Cross-device sync

### Growth Features (Post-MVP)

- Browser extension for article capture
- Readwise / Pocket integration
- Search within generated content
- Gamification (streaks, hours learned)
- Additional voice options
- Native mobile apps

### Vision (Future)

- Creator monetization
- Public discovery (opt-in)
- Multi-language support
- API for integrations

## User Journeys

### Journey 1: James — The Thesis Crunch (Primary User, Happy Path)

**The Persona:** James is 22, a grad student in behavioral economics. His thesis is due in 8 weeks. His advisor just added 30 more papers to his reading list. He runs 5K every morning — 40 minutes of "dead time."

**Opening Scene:** Sunday night. James stares at 30 papers, each 20-40 pages. At his reading speed, that's 60+ hours he doesn't have. A friend texts: "Dude, try Disona."

**Rising Action:** James uploads his first 5 papers. The app parses chapters instantly. He clicks "Generate Blitz Mode." 60 seconds later: push notification. He hits play during his morning run.

**Climax:** Wednesday morning. He's "read" 8 papers in 3 days — during runs. He opens Podcast Mode on three contradicting papers. AI hosts debate the methodology. James finally *gets* the literature.

**Resolution:** 6 weeks later, thesis submitted. 47 sources cited. He read maybe 10 the traditional way. He shares his "Behavioral Econ Foundations" playlist with his cohort. Three sign up.

---

### Journey 2: Priya — The Busy Parent (Primary User, Different Context)

**The Persona:** Priya is 35, engineering lead, two kids under 7. Her "free time" is 30 minutes driving to work and 45 minutes at the 5am gym. She hasn't finished a book in 2 years.

**Opening Scene:** 5:15am, treadmill. Her phone shows a newsletter recommending 5 leadership books. She screenshots it, knowing she'll never read them.

**Rising Action:** She uploads "The Manager's Path." Selects Blitz Mode. 8 minutes later, she's through 3 chapter summaries. Chapter 4 sounds relevant — she taps "Listen Full Chapter."

**Climax:** Two weeks later, in a 1:1, she quotes the book naturally. She didn't just hear it — she absorbed it. In gym sessions and car rides.

**Resolution:** She upgrades to €20 tier. Creates a "Leadership Stack" folder with 6 books. Her "Someday" folder is empty for the first time in years.

---

### Journey 3: Marcus — The Recipient (Secondary User, Growth Loop)

**The Persona:** Marcus is 29, product designer. His friend James texts him a Disona link: "Listen to this podcast about loss aversion."

**Opening Scene:** Marcus clicks the link. He doesn't want to sign up. He just wants to hear what James sent.

**Rising Action:** The link opens in his browser. A simple player, no login required. Two AI voices discuss the paper. Marcus pauses at 6:42 to screenshot a point.

**Climax:** A week later, James shares more. Marcus has listened to 3 shares. The app prompts: "Create a free account to save your history." He signs up. Explores. Sees "Upload your own PDF." Upgrade prompt. He thinks of the UX book on his desk.

**Resolution:** Marcus becomes a paying creator. He shares his own Blitz Mode content with his design team.

---

### Journey 4: Alex — The Skeptical New User (Onboarding)

**The Persona:** Alex is 27, consultant who saw Disona on Product Hunt. Skeptical. Has 15 minutes before a meeting.

**Opening Scene:** Alex lands on the homepage. Clicks "Try it Free." Sign-up takes 30 seconds (Google OAuth).

**Rising Action:** The app offers "Try a sample" — a pre-loaded TED talk transcript. Alex picks Blitz Mode. First section ready in 12 seconds. The voice is natural, not robotic. The summary is accurate.

**Climax:** 4 minutes in, Alex realizes: *This actually works.* They find a 30-page industry report they've been avoiding. Upload. Generate. First section in 45 seconds.

**Resolution:** Alex finishes the report during dishes and a walk. They upgrade to Basic before the trial ends.

---

### Journey 5: Dana — The Broken Upload (Edge Case, Error Recovery)

**The Persona:** Dana is a paying user who uploads a scanned PDF — 400 pages, poor OCR quality.

**Opening Scene:** Processing takes longer than usual. After 3 minutes: "We had trouble with some pages. 312 of 400 pages processed."

**Rising Action:** The app offers options: generate from readable sections, re-upload cleaner version, or contact support. Dana generates from readable sections.

**Climax:** Chapter 7 has a gap. Dana submits feedback. The app offers partial credit and tips for better scans.

**Resolution:** Dana re-scans the problem chapter, uploads just that section. Disona merges it with existing audio. Graceful recovery.

---

### Journey Requirements Summary

| Capability | Revealed By Journey |
|------------|---------------------|
| PDF upload + chapter detection | James, Priya, Alex |
| Blitz Mode (summaries + drill-down) | James, Priya, Alex |
| Podcast Mode (multi-voice debate) | James |
| Audiobook Mode (full narration) | Priya, Alex |
| Progressive generation | James, Alex |
| Push notifications | James |
| Playlist/queue | James, Priya |
| Resume from exact second | Priya, Alex, Marcus |
| Folder organization | Priya |
| Sharing (link-based) | James, Priya, Marcus |
| Anonymous playback | Marcus |
| Free account (listen-only) | Marcus |
| Upgrade flow | Priya, Marcus, Alex |
| Sample content (onboarding) | Alex |
| OAuth signup | Alex |
| OCR error handling | Dana |
| Partial processing | Dana |
| User feedback / support | Dana |

## Domain-Specific Requirements

### Compliance & Regulatory

**GDPR Compliance (Mandatory)**
- All user data stored exclusively in EU data centers
- Admin endpoints for:
  - Fetching user data (data portability request)
  - Deleting user data (right to be forgotten)
- Privacy policy clearly stating EU-only storage
- Cookie consent for analytics/tracking

**Copyright Handling**
- Terms of Service: user responsible for uploaded content
- Private sharing only — no public distribution

**Accessibility**
- WCAG 2.1 AA compliance for PWA interface
- Keyboard-navigable audio player
- Transcripts: NOT in MVP (future feature)

### Technical Constraints

**Data Storage**
- EU-only hosting (AWS eu-central, GCP europe-west, or equivalent)
- Encryption at rest for all user uploads and generated audio
- Generated audio retained indefinitely until user deletes
- Secure deletion when user removes content

**Privacy**
- No training AI models on user content
- Clear data retention policy in ToS
- User controls their data lifecycle

**Authentication**
- OAuth (Google, Apple) for frictionless signup
- Session management for cross-device sync
- Sharing links: private, user-controlled

### Admin Requirements

| Endpoint | Purpose | GDPR Article |
|----------|---------|--------------|
| `GET /admin/users/{id}/data` | Export all user data | Art. 20 (Portability) |
| `DELETE /admin/users/{id}` | Delete all user data | Art. 17 (Erasure) |
| Audit logging | Track admin actions | Art. 30 (Records) |

### Risk Mitigations

| Risk | Mitigation |
|------|------------|
| GDPR violation | EU-only hosting, admin endpoints, clear ToS |
| Copyright claims | User responsibility in ToS, private sharing only |
| Data breach | Encryption at rest, secure deletion, audit logs |
| Accessibility lawsuit | WCAG 2.1 AA from launch |

## Innovation & Novel Patterns

### Detected Innovation Areas

**1. The "Depth on Demand" Pattern**
No existing solution lets you seamlessly move between summary and full content. Blinkist is summary-only. Audible is full-only. Disona bridges them — skim the structure, then dive deep where it matters.

**2. Creator-First Audio Learning**
Existing tools are consumer-first (here's our catalog) or creation-heavy (complex editing). Disona makes creation instant and invisible — upload → audio. The user is a curator, not a producer.

**3. Progressive Audio Generation**
Typical flow: upload → wait → listen. Disona: upload → listen immediately (first chapter) while rest generates. This eliminates the "processing penalty" that kills engagement.

**4. The Format Mismatch Framing**
Competitors sell "save time" or "learn faster." Disona reframes: "Your reading list isn't a time problem — it's a format problem." This is a positioning innovation, not just a feature innovation.

**5. Sharing as Gifting (Not Spamming)**
Most sharing features fail because shared content isn't valuable enough. Disona's shares are curated, processed, valuable — a gift, not an ad. This creates genuine network effects.

### Market Context & Competitive Landscape

| Innovation | Competitive Response Risk | Defensibility |
|------------|---------------------------|---------------|
| Depth on demand | Medium — NotebookLM could add | UX execution, mobile-first |
| Creator-first | Low — requires product rethink | Positioning moat, community |
| Progressive generation | Medium — technical | First-mover advantage |
| Format mismatch framing | Low — positioning is defensible | Brand ownership |
| Sharing as gifting | Medium — easy to copy | Network effects compound |

### Validation Approach

| Innovation | How to Validate |
|------------|-----------------|
| Depth on demand | A/B test: do users who "go deep" retain better? |
| Creator-first | Track: uploads per user, repeat creation rate |
| Progressive generation | Measure: time-to-first-play, drop-off during processing |
| Format mismatch | Marketing test: which pitch converts better? |
| Sharing as gifting | Track: shares per creator, recipient conversion rate |

### Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Depth on demand isn't valued | Default to Blitz mode; make "go deep" discoverable |
| Creator friction too high | Sample content for instant demo; optimize upload flow |
| Progressive generation hard | Start with chapter-level streaming; optimize later |
| Positioning doesn't resonate | Test multiple pitches; iterate on messaging |
| Sharing doesn't drive growth | Add incentives (extra hours for referrals) |

## Web App (PWA) Specific Requirements

### Project-Type Overview

Disona is a **Single Page Application (PWA)** optimized for mobile-first usage. The app is designed for modern evergreen browsers with offline capability, focusing on audio playback experience over SEO.

### Technical Architecture Considerations

**Application Type:** SPA (Single Page Application)
- Client-side routing for seamless navigation
- Service Worker for offline support and caching
- Web App Manifest for "Add to Home Screen"

**Real-time Strategy:** Polling + Push Notifications (MVP)
- Push notifications for "audio ready" alerts
- Polling for progress sync across devices
- WebSocket/SSE considered for future (post-MVP)

### Browser Support Matrix

| Browser | Version | Priority |
|---------|---------|----------|
| Chrome (Desktop) | Latest 2 versions | High |
| Chrome (Android) | Latest 2 versions | **Critical** |
| Safari (iOS) | Latest 2 versions | **Critical** |
| Safari (Desktop) | Latest 2 versions | Medium |
| Firefox | Latest 2 versions | Medium |
| Edge | Latest 2 versions | Low |

### Responsive Design Requirements

| Breakpoint | Target | Priority |
|------------|--------|----------|
| Mobile (< 768px) | Primary experience | **Critical** |
| Tablet (768px - 1024px) | Good experience | High |
| Desktop (> 1024px) | Functional experience | Medium |

### Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| First Contentful Paint | < 1.5s | Fast perceived load |
| Time to Interactive | < 3s | Quick first interaction |
| Largest Contentful Paint | < 2.5s | Core Web Vitals |
| Audio playback start | < 1s after tap | Critical UX |
| Offline load | < 500ms | PWA requirement |

### PWA Requirements

- Service Worker for offline audio playback
- Cache-first strategy for audio files
- Background sync for progress updates
- Installable via "Add to Home Screen"
- Web App Manifest with icons

### SEO Strategy

| Component | SEO Approach |
|-----------|--------------|
| App (authenticated) | Not indexed (noindex) |
| Landing/Marketing site | Future — separate implementation |
| Shared content links | OG tags for social preview cards |

### Accessibility Level

**Target:** WCAG 2.1 AA (baseline)

| Feature | Level | Notes |
|---------|-------|-------|
| Keyboard navigation | Nice-to-have | Not critical for MVP |
| Screen reader support | Basic | Audio player must be accessible |
| Color contrast | AA | Standard compliance |
| Focus indicators | Required | For keyboard users |

### Audio Implementation

- HTML5 Audio API for playback
- MediaSession API for lock screen controls
- Background audio playback (critical for mobile)
- Resume-from-second stored locally + synced to cloud

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Experience MVP
- Deliver the complete "skim → deep dive → discuss" experience
- Limited to one input type (PDF) to reduce complexity
- Prove that depth-on-demand is the differentiator

**Core Hypothesis:**
> "Users will pay for the ability to control their learning depth — skim when they want, go deep when it matters."

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**
- ✅ James (Grad student) — Full journey
- ✅ Priya (Busy parent) — Full journey
- ✅ Marcus (Recipient) — Full journey
- ✅ Alex (New user) — Full journey
- ⚠️ Dana (Error recovery) — Basic handling

**Must-Have Capabilities:**

| Capability | Rationale |
|------------|-----------|
| PDF upload + chapter detection | Core input mechanism |
| Audiobook Mode | Complete experience |
| Blitz Mode (summaries + drill-down) | Key differentiator |
| Podcast Mode (multi-host debate) | Wow factor |
| Progressive generation | Eliminates processing penalty |
| Resume from exact second | Critical for commute |
| Playlist/queue | Continuous listening |
| Variable playback speed (0.5x-3x) | Expected feature |
| Push notifications | "Audio ready" alerts |
| Private sharing (link-based) | Growth loop |
| Free tier (listen-only) | Conversion funnel |
| Two paid tiers (€5/€20) | Revenue |
| OAuth signup (Google) | Frictionless onboarding |
| Sample content | First-time demo |
| Folder organization | Content management |
| Offline download | Commute reliability |
| Cross-device sync | Resume anywhere |

**Explicitly NOT in MVP:**

| Feature | Phase |
|---------|-------|
| Article/URL input | Phase 2 |
| Browser extension | Phase 2 |
| Readwise/Pocket integration | Phase 2 |
| Transcripts | Phase 2 |
| Search within audio | Phase 2 |
| Gamification (streaks) | Phase 2 |
| Additional voices | Phase 2 |
| Creator monetization | Phase 3 |
| Public discovery | Phase 3 |
| Native mobile apps | Phase 3 |
| Multi-language | Phase 3 |
| API for integrations | Phase 3 |

### Post-MVP Roadmap

**Phase 2: Growth (Months 6-12)**
- Article/URL input
- Browser extension
- Transcripts
- Search within content
- Gamification
- Additional TTS voices
- Apple OAuth
- Referral incentives

**Phase 3: Expansion (Year 2+)**
- Creator monetization
- Public discovery (opt-in)
- Native iOS/Android apps
- Multi-language TTS
- API & integrations
- Team/Enterprise tier

### Risk Mitigation Strategy

| Risk Type | Risk | Mitigation |
|-----------|------|------------|
| Technical | Chapter detection fails | Semantic chunking fallback |
| Technical | TTS quality disappoints | Set expectations; iterate |
| Technical | Podcast mode hallucinates | Preview before share |
| Market | NotebookLM adds mobile | Move fast; build brand |
| Market | Pitch doesn't resonate | A/B test messaging |
| Resource | Fewer resources | Cut to Blitz + Audiobook only |
| Resource | Higher TTS costs | Reduce free tier; optimize |

### Resource Requirements

**MVP Team Size:**
- 1 Full-stack developer
- 1 ML/AI engineer
- 0.5 Designer
- 0.5 Product

**Timeline:** 3-4 months to MVP launch

## Functional Requirements

### 1. Content Ingestion (FR1-FR7)

- **FR1:** User can upload PDF files for processing
- **FR2:** System detects and extracts chapters/sections from PDFs
- **FR3:** System applies semantic chunking when chapter markers absent
- **FR4:** User can organize content into folders
- **FR5:** System pre-parses files before audio generation request
- **FR6:** User can view detected structure of uploaded content
- **FR7:** System handles partial processing gracefully

### 2. Audio Generation (FR8-FR15)

- **FR8:** User can generate Audiobook Mode (full narration)
- **FR9:** User can generate Blitz Mode (summaries + key points)
- **FR10:** User can generate Podcast Mode (multi-host discussion)
- **FR11:** System generates first chapter within 60 seconds
- **FR12:** System continues generating while user listens
- **FR13:** User receives push notification when generation complete
- **FR14:** System tracks and displays generation progress
- **FR15:** User can view estimated time remaining

### 3. Audio Playback (FR16-FR26)

- **FR16:** User can play, pause, and stop audio
- **FR17:** User can skip forward and backward
- **FR18:** User can adjust playback speed (0.5x-3x)
- **FR19:** System remembers exact playback position
- **FR20:** User can resume from exact position across sessions
- **FR21:** User can resume from exact position across devices
- **FR22:** User can view and navigate chapter markers
- **FR23:** User can view key point timestamps (Blitz Mode)
- **FR24:** User can switch from summary to full chapter ("go deep")
- **FR25:** System displays lock screen controls
- **FR26:** System continues playback in background

### 4. Content Library & Organization (FR27-FR35)

- **FR27:** User can view all content in library
- **FR28:** User can create, rename, delete folders
- **FR29:** User can move content between folders
- **FR30:** User can create playlists/queues
- **FR31:** User can reorder playlist items
- **FR32:** System auto-plays next item in playlist
- **FR33:** User can download for offline playback
- **FR34:** User can delete content and audio
- **FR35:** User can view listening history and progress

### 5. Sharing & Collaboration (FR36-FR41)

- **FR36:** User can generate shareable links
- **FR37:** User can share with specific individuals
- **FR38:** Recipient can access without logging in
- **FR39:** Recipient can create free account
- **FR40:** System tracks who has access to shared content
- **FR41:** User can revoke access to shared content

### 6. User Accounts & Authentication (FR42-FR46)

- **FR42:** User can sign up using Google OAuth
- **FR43:** User can sign in and sign out
- **FR44:** User can view and edit profile
- **FR45:** User can delete account and all data
- **FR46:** System maintains session across devices

### 7. Subscription & Billing (FR47-FR54)

- **FR47:** User can view subscription tiers
- **FR48:** User can subscribe to paid tier
- **FR49:** User can upgrade or downgrade
- **FR50:** User can cancel subscription
- **FR51:** System tracks hours used vs. available
- **FR52:** User can view usage dashboard
- **FR53:** System enforces limits by tier
- **FR54:** Free users can only listen to shared content

### 8. Onboarding & Discovery (FR55-FR58)

- **FR55:** New user can access sample content
- **FR56:** New user can try all three modes on sample
- **FR57:** System guides first upload flow
- **FR58:** System prompts for push notification permission

### 9. Admin & Compliance (FR59-FR63)

- **FR59:** Admin can export user data (GDPR)
- **FR60:** Admin can delete user data (GDPR)
- **FR61:** System logs admin actions
- **FR62:** System displays privacy policy and ToS
- **FR63:** System obtains cookie consent

### 10. Error Handling & Support (FR64-FR68)

- **FR64:** User can report issues with audio
- **FR65:** System displays clear error messages
- **FR66:** User can retry failed generation
- **FR67:** User can re-upload sections to fix failures
- **FR68:** System offers partial credit for failures

**Total: 68 Functional Requirements**

## Non-Functional Requirements

### Performance

| Requirement | Target | Rationale |
|-------------|--------|-----------|
| **NFR-P1:** First chapter audio available | < 60 seconds | Progressive generation |
| **NFR-P2:** Audio playback start | < 1 second after tap | Critical UX |
| **NFR-P3:** App load time (cached) | < 500ms | PWA experience |
| **NFR-P4:** App load time (cold) | < 3 seconds | First Contentful Paint |
| **NFR-P5:** Full book processing (300 pages) | < 10 minutes | User expectation |
| **NFR-P6:** Chapter navigation response | < 200ms | Seamless listening |
| **NFR-P7:** Cross-device sync latency | < 5 seconds | Resume reliability |
| **NFR-P8:** Push notification delivery | < 30 seconds | Timely engagement |

### Security

| Requirement | Target | Rationale |
|-------------|--------|-----------|
| **NFR-S1:** Data encryption at rest | AES-256 | GDPR compliance |
| **NFR-S2:** Data encryption in transit | TLS 1.3 | Industry standard |
| **NFR-S3:** Authentication | OAuth 2.0 | Standard protocol |
| **NFR-S4:** Session timeout | 30 days inactive | Balance security/convenience |
| **NFR-S5:** Data residency | EU-only | GDPR compliance |
| **NFR-S6:** User data deletion | Within 72 hours | GDPR Article 17 |
| **NFR-S7:** Admin action logging | All operations logged | Audit trail |
| **NFR-S8:** No AI training on user content | Contractual commitment | Privacy assurance |

### Scalability

| Requirement | Target | Rationale |
|-------------|--------|-----------|
| **NFR-SC1:** Concurrent users | 500 simultaneous (MVP) | 25% of target |
| **NFR-SC2:** Audio storage | 100 hours per user avg | Pro tier usage |
| **NFR-SC3:** Generation queue | 50 concurrent jobs | Peak usage |
| **NFR-SC4:** Growth headroom | 10x without re-architecture | Planning for success |
| **NFR-SC5:** CDN delivery | Global edge caching | Low-latency playback |

### Reliability

| Requirement | Target | Rationale |
|-------------|--------|-----------|
| **NFR-R1:** Uptime SLA | 99.5% | ~3.6 hours/month max |
| **NFR-R2:** Data durability | 99.99% | Content must not be lost |
| **NFR-R3:** Offline playback | Full functionality | Commute reliability |
| **NFR-R4:** Resume accuracy | ±1 second | Critical UX |
| **NFR-R5:** Graceful degradation | Partial > failure | Error recovery |
| **NFR-R6:** Background sync | Queue offline updates | Offline-first |

### Accessibility

| Requirement | Target | Rationale |
|-------------|--------|-----------|
| **NFR-A1:** Color contrast | WCAG 2.1 AA (4.5:1) | Basic compliance |
| **NFR-A2:** Screen reader support | Audio controls accessible | Core accessibility |
| **NFR-A3:** Focus indicators | All interactive elements | Keyboard support |
| **NFR-A4:** Touch targets | Minimum 44x44px | Mobile accessibility |

**Total: 31 Non-Functional Requirements**
