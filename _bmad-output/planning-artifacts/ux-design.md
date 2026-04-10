---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-03-core-experience", "step-04-emotional", "step-05-inspiration", "step-06-design-system", "step-07-final-review"]
inputDocuments:
  - "_bmad-output/planning-artifacts/prd.md"
  - "_bmad-output/planning-artifacts/product-brief-disona.md"
  - "_bmad-output/planning-artifacts/product-brief-disona-distillate.md"
status: complete
---

# UX Design Specification - Disona

**Author:** Gotha
**Date:** 2026-04-08

---

## Executive Summary

### Project Vision

Disona is a mobile-first PWA that transforms written content into intelligent audio learning experiences. The core UX insight is that users don't lack time — they lack content in a format that fits their available attention (commutes, workouts, chores).

The product enables "depth on demand": users can skim chapter summaries (Blitz Mode), listen to full narration (Full Mode), or hear ideas debated (Podcast Mode) — and seamlessly transition between depths based on interest.

### Target Users

**Primary: Graduate students and knowledge workers with real learning pressure**
- Have 30+ papers or books they need to consume
- Already use audio during commutes and workouts
- Need to triage content: quickly identify what's worth deep attention

**Secondary: Recipients of shared content**
- Friends and colleagues who receive shared audio
- Zero-friction entry: can listen without signing up (with email-locked links)

**User Context:**
- Hands busy, ears free (commuting, exercising, chores)
- Often one-handed mobile use
- Frequently offline (subways, parking garages, gyms)
- Need to resume exactly where they left off

### Key Design Challenges

1. **Skim → Deep Transition:** Seamless, not jarring mode switch
2. **Progressive Generation Feedback:** "Already started" not "still waiting"
3. **Learning-Optimized Audio Player:** Chapter markers, key points, speed control
4. **Mobile-First, Thumb-Reachable:** One-handed commute use
5. **Offline-First Reliability:** Downloaded content works flawlessly

---

## Content Hierarchy

### Three-Level Structure

| Level | Content | Duration |
|-------|---------|----------|
| **L1 Summary** | Chapter overview | ~1 min |
| **L2 Summaries** | Key point summaries (4-6 per chapter) | ~45 sec each |
| **Full Chapter** | Complete narration, segmented by key points | ~25-40 min |

### Navigation Flow (Blitz Mode)

```
L1 Summary → L2 (KP1) → L2 (KP2) → L2 (KP3) → L2 (KP4) → Next Chapter L1
                 ↓           ↓           ↓           ↓
            [Go Deep]   [Go Deep]   [Go Deep]   [Go Deep]
                 ↓           ↓           ↓           ↓
            Full @ 0%   Full @ 25%  Full @ 35%  Full @ 70%
```

### Go Deep Behavior

- **Tap "Go Deep"** → Immediate jump to full chapter at relevant timestamp
- **After full chapter ends** → Return to Blitz Mode (next L1 summary)
- **"Stay in Full Mode" checkbox** → If checked, continues in Full Mode permanently
- Default: unchecked (temporary deep mode)

---

## Two-Stage Processing Pipeline

### Stage 1: Content Analysis (Automatic, Metered by Upload Quota)

1. PDF parsing + title extraction
2. Chapter detection (bookmarks → TOC → headings → semantic → size fallback)
3. Key point segmentation (adjacent sentence similarity via embeddings)
4. L1/L2 summary generation (text only)

### Stage 2: Audio Generation (On-Demand, Metered by Audio Quota)

- User must explicitly click "Generate Audio"
- Options: single chapter or entire book
- Quota check before generation
- Push notification when complete

### Subscription Model

| Tier | Upload Quota | Audio Quota | Price |
|------|--------------|-------------|-------|
| Free | 0h | 0h | €0 |
| Standard | 50h | 20h | €5/mo |
| Pro | 250h | 100h | €20/mo |
| Design Partner | ∞ | ∞ | €0 |

---

## Onboarding Flow

### Sample Content (Pre-processed, Legally Cleared)

| # | Title | Category |
|---|-------|----------|
| 1 | "Attention Is All You Need" | Scientific Paper (arXiv) |
| 2 | "The Adventures of Tom Sawyer" | Public Domain |
| 3 | "Free Culture" by Lawrence Lessig | Copyleft (CC BY-NC) |

### Access Tiers

| Tier | Access |
|------|--------|
| **Anonymous** | Sample chapters 1-3 |
| **Free Account** | Full samples + shared content |
| **Paid Account** | Upload own content + generate audio |

### Conversion Gates

| Gate | Trigger |
|------|---------|
| **Registration** | Tap on Chapter 4+ of sample |
| **Payment** | Tap "Upload PDF" |

---

## Library & Playback Queue

### Library Organization

- **Folders** — User-created for organization
- **Special folders** — "Shared With Me", "Unfiled"
- **Filter tabs** — All, In Progress, Completed, Shared
- **Per-book progress** — Chapter, mode, position, last listened

### Two Listening Modes (Per-Book)

| Mode | Behavior | After Chapter |
|------|----------|---------------|
| **Blitz Mode** | L1 → L2s with "Go Deep" option | Next L1 summary |
| **Full Mode** | Complete narration, no summaries | Next full chapter |

### Queue Capabilities

- Add: entire book (Blitz or Full), single chapter, selected chapters
- Position: Play Next or End of Queue
- Reorder: drag to reorder, swipe to remove
- Download: one button for entire queue (~240 MB per book)

### End of Book

- Stop playback (no auto-play next book)
- Show completion screen with "Play Next" option

---

## Sharing & Gifting

### Email-Locked Sharing

- Each share bound to specific email address
- Link contains hashed email in token
- Recipient must sign in with matching email
- Cannot forward links to others

### What Can Be Shared

- Entire book (Blitz + Full)
- Single chapter
- Selected chapters
- Personal message from sender

### Notifications

| Event | Who Gets Notified |
|-------|-------------------|
| Share created | Recipient (email + push) |
| Recipient starts listening | Owner (push) |
| Recipient finishes book | Owner (push) |
| Owner deletes book | Recipient (push + email) |

### Offline Shared Content

- If recipient downloaded content before owner deletes
- Content remains playable offline
- Marked as "offline only"
- Cannot re-download once deleted

---

## Audio Player Design

### Player States

| View | Content |
|------|---------|
| **Mini Player** | Book title, chapter, play/pause, progress bar |
| **Expanded Player** | Chapter title, live transcript, transport, speed |
| **Lock Screen** | Book cover, title, 5 buttons, Go Deep |

### Transport Controls (5 Buttons)

```
⏮️     ◀◀     ▶️⏸     ▶▶     ⏭️
prev   -15s   play    +15s   next
seg                          seg
```

Same 5 buttons on lock screen via MediaSession API.

### Live Transcript

- Only in Expanded Player
- Current sentence highlighted
- Auto-scrolls to keep current text visible
- User can manually scroll

### Speed Control

- Range: 0.5x to 3x
- Presets: 0.5x, 0.75x, 1x, 1.25x, 1.5x, 1.75x, 2x, 2.5x, 3x

### Browse Views

| Mode | View |
|------|------|
| **Full Mode** | List of chapters with progress |
| **Blitz Mode** | Tree: L1 summaries with nested L2 key points |

- Mini player at bottom
- Auto-scroll to highlight current playing item
- Mode switch button available

### Temporary Deep Mode

- Checkbox: "Stay in Full Mode" (unchecked by default)
- Unchecked: After chapter → return to Blitz
- Checked: After chapter → stay in Full mode permanently

---

## Offline Download

### Strategy: Download Everything

- All content (L1, L2, Full chapters) ~240 MB per book
- Download priority: summaries first, then full chapters
- User can start listening before full download completes

### Download Queue

- One button to download entire queue
- Shows total size estimate
- WiFi-only option available

---

## MVP Minimal Decisions

### Account & Settings

- Email display, subscription tier, quota display
- Sign out, delete account (GDPR)
- No advanced settings for MVP

### Push Notifications (3 Triggers Only)

| Trigger | Message |
|---------|---------|
| Audio ready | "Your audio is ready!" |
| Received share | "[Name] shared [Book] with you" |
| Owner deleted | "[Book] is no longer available" |

### Error States

- Generic error message + retry button
- Graceful degradation where possible

### Search

- Basic text search (book titles, chapter names)
- No full-text search within content for MVP

### Podcast Mode

- Deferred to post-MVP

---

## Core Experience Definition

### Experience Principles (Prioritized)

| Priority | Principle | Description |
|----------|-----------|-------------|
| **P0** | **Depth on Demand** | User controls how deep they go — never forced |
| **P0** | **Your Content, Your Format** | User uploads what matters to them |
| **P0** | **Trust the System** | Progress saved, offline works, nothing lost |
| **P1** | **Hands-Free Learning** | Lock screen controls, auto-advance, resume-from-second |
| **P2** | **Share as Gift** | Sharing feels personal and valuable, not spammy |
| **P2** | **Zero Wait, Start Listening** | Progressive generation, no "processing" penalty |

### Anti-Principles (What We Do NOT Do)

| # | Anti-Principle | We Do NOT... |
|---|----------------|--------------|
| 1 | **No catalog** | Build or curate content |
| 2 | **No gamification** | Add streaks, points, badges |
| 3 | **No interruptions** | Show ads, upsells, or popups during playback |
| 4 | **No eyes required** | Require visual attention to listen |
| 5 | **User controls depth** | Auto-switch modes or skip content |
| 6 | **No referral spam** | Make sharing feel like marketing |
| 7 | **No content mining** | Train AI on user uploads |
| 8 | **Offline is first-class** | Require always-online |
| 9 | **No dark patterns** | Optimize for engagement metrics |
| 10 | **Value, not price** | Compete on price |
| 11 | **Focus on core audience** | Chase every market |

### Edge Case Handling

| Principle | Failure Scenario | Response |
|-----------|------------------|----------|
| **Depth** | No chapters detected | Size-based fallback → Full Mode only |
| **Depth** | Go Deep unavailable | Queue it, continue with summaries |
| **Your Content** | Unreadable PDF | Clear error, don't charge quota |
| **Your Content** | Partial parse | Continue with what works |
| **Trust** | Sync conflict | Furthest progress wins |
| **Trust** | Progress lost | Restore from backup, apologize, compensate |
| **Hands-Free** | Bluetooth disconnect | Pause immediately, don't auto-resume |
| **Sharing** | Email mismatch | Clear error, "Contact Sender" option |
| **Speed** | Slow generation | Honest estimate, push notification |

**Universal Rules:**
1. Never charge quota for failed operations
2. Always provide a next step (retry, skip, contact support)
3. Explain WHY something failed
4. Partial success > total failure
5. Apologize and compensate for trust-breaking failures

### Measuring Principles

| Principle | Key Metric | Target |
|-----------|------------|--------|
| **Depth** | Go Deep rate | >20% |
| **Your Content** | Upload success | >90% |
| **Trust** | Resume accuracy | ±2 sec |
| **Hands-Free** | Content completion rate | >70% |
| **Sharing** | Quote → Expand rate | >40% |
| **Speed** | Time to Ch1 | <90 sec |

### Enhanced Sharing (Quote-Level)

| Share Type | Description | Expected % |
|------------|-------------|------------|
| **Quote** | Specific highlighted text | 30-40% |
| **L2 Summary** | Single key point | 20-30% |
| **L1 Summary** | Chapter overview | 10-20% |
| **Full Chapter** | One chapter | 15-25% |
| **Entire Book** | All chapters | 5-10% |

### Competitive Strategy

**Moats (Ranked):**
1. **Niche Ownership** (grad students, researchers) — High
2. **Community** (study groups, professor→student) — High
3. **UX Polish** (death by 1000 details) — Medium
4. **Switching Cost** (progress, library, shares) — Growing

**North Star:** Be so good for grad students that when Google launches, our users say "Yeah, but Disona just GETS me."

---

## Internationalization

### MVP (English Only)

| Aspect | Decision |
|--------|----------|
| UI language | English |
| Source content | English |
| Output audio | English |
| Architecture | Ready for multi-language |

### V2+ Translation Feature

**Killer Feature:** Multi-language input → Single target output

Example: English + French + Russian papers → Bulgarian audio

### Language Roadmap

| Phase | Timeline | Languages |
|-------|----------|-----------|
| **MVP** | Month 0 | English |
| **V2.0** | Month 9 | German, French, Spanish |
| **V2.1** | Month 11 | Portuguese, Italian |
| **V2.2** | Month 13 | Polish, Turkish |
| **V2.3** | Month 15 | Romanian, Bulgarian, Greek |
| **V2.4** | Month 17 | Czech, Hungarian |

### Strategic Markets

| Market | Strategy |
|--------|----------|
| **Western EU** | Compete with localized Blinkist |
| **Eastern EU** | First-mover advantage, underserved markets |
| **Diaspora** | Target Poles in UK, Turks in DE, Bulgarians abroad |

---

## Emotional Response

### Desired Emotions

| Emotion | Priority | Description |
|---------|----------|-------------|
| **In Control** | P0 | "I decide how deep to go" |
| **Accomplished** | P0 | "I actually finished the book" |
| **Trusting** | P0 | "My progress is safe, it just works" |
| **Efficient** | P1 | "I'm tackling my reading backlog" |
| **Connected** | P2 | "This fits into my life" |

### Emotions to Avoid

| Negative Emotion | How We Prevent It |
|------------------|-------------------|
| **Guilt** | No gamification, no "you missed X days" |
| **Overwhelm** | Simple UI, clear next steps |
| **Anxiety** | Trust principle, clear progress |
| **FOMO** | No catalog to browse, only YOUR content |
| **Pressure** | No streaks, no leaderboards |

### 7 Critical Emotional Moments

| # | Moment | Emotion Shift | Key Design Element |
|---|--------|---------------|-------------------|
| 1 | **First Listen** | Skeptical → Impressed | Audio in <500ms, premium voice |
| 2 | **Upload** | Anxious → Reassured | First chapter title = anxiety breaker |
| 3 | **Go Deep** | Curious → Satisfied | Smooth crossfade, <2 sec |
| 4 | **Completion** | Accomplished → Proud | Simple celebration, no gamification |
| 5 | **Receive Share** | Curious → Touched | Personal message is the hero |
| 6 | **Return After Break** | Disoriented → Relieved | Exact position, no guilt |
| 7 | **Error/Failure** | Frustrated → Understood | Explain why, offer path |

### Emotional Differentiation

| Competitor | Their Feeling | Disona's Counter |
|------------|---------------|------------------|
| **Blinkist** | "I cheated on my reading" | "I chose my depth strategically" |
| **Audible** | "Guilty about unfinished books" | "I actually finish books now" |
| **NotebookLM** | "I used a tool" | "Disona fits into my life" |

**Core emotional promise:**
> "Others make you feel like you're failing at reading. Disona makes you feel like you're winning at learning."

### Tone of Voice

| We Say | We Don't Say |
|--------|--------------|
| "Pick up where you left off" | "You've been gone for X days" |
| "45% complete" | "6 hours remaining" |
| "Go deeper" | "See the full version" |
| "Share with a friend" | "Invite to earn rewards" |
| "Your book" | "This title" |
| "Ready when you are" | "Don't lose your streak!" |

---

## UX Pattern Inspiration

### Spotify (Audio Player)

| Pattern | Adoption |
|---------|----------|
| Persistent mini player | ✅ Always visible |
| Seekable progress bar | ✅ Tap/drag to seek |
| Time on both sides | ✅ Current + total |
| Large central play button | ✅ Thumb-friendly |
| Swipe up/down gestures | ✅ Expand/collapse |
| Album art as anchor | ✅ Book cover |

### Pocket Casts (Queue, Speed, Progress)

| Pattern | Adoption |
|---------|----------|
| Drag-to-reorder queue | ✅ Same |
| Swipe-to-remove | ✅ Same |
| Speed presets | ✅ 0.5x to 3x |
| Speed indicator in mini player | ✅ Show current |
| Time remaining | ✅ "23 min left" |
| In Progress filter | ✅ Library filter |

### Apple Books (Progress Tracking)

| Pattern | Adoption |
|---------|----------|
| Progress bar on thumbnail | ✅ Visual at-a-glance |
| "Reading Now" section | ✅ "Continue Listening" |
| Multiple progress formats | ✅ Chapter, %, time |
| Chapter jump navigation | ✅ Key point tree |
| Streaks/goals | ❌ Anti-gamification |

### Linear (Keyboard UX — Desktop/PWA)

| Shortcut | Action |
|----------|--------|
| `Space` | Play/Pause |
| `D` | Go Deep |
| `B` | Switch to Blitz |
| `← / →` | Skip 15 seconds |
| `[ / ]` | Adjust speed |
| `G L` | Go to Library |
| `G Q` | Go to Queue |
| `⌘K` | Command palette |
| `?` | Show shortcuts |

### Anti-Patterns (Avoided)

| Pattern | Source | Reason |
|---------|--------|--------|
| Streaks | Apple Books | Guilt |
| Goals/targets | Apple Books | Pressure |
| Social features | Spotify | Distraction |
| Algorithmic recs | Spotify | No catalog |

---

## Design System

### Technical Foundation

| Layer | Tool | Purpose |
|-------|------|---------|
| **Styling** | Tailwind CSS | Utility-first, responsive |
| **Primitives** | Radix UI | Accessible dialogs, dropdowns |
| **Icons** | Lucide | Consistent, lightweight |
| **Animation** | Framer Motion | Smooth transitions |
| **Custom** | Our components | Player, transcript, tree |

### Visual Tone: Bold & Energetic (Spotify-inspired)

- High contrast, vibrant accents
- Bold headlines, confident typography
- Generous spacing
- Smooth, purposeful animations
- Dark mode as primary experience

### Color Palette

| Token | Dark Mode | Light Mode | Usage |
|-------|-----------|------------|-------|
| `background` | #0a0a0a | Gray-100 | Page |
| `surface` | #171717 | White | Cards |
| `surface-elevated` | #262626 | White | Player, modals |
| `primary` | #1DB954 | Green-500 | CTAs, play |
| `text-primary` | White | Gray-900 | Main text |
| `text-secondary` | Gray-400 | Gray-500 | Secondary |
| `blitz-accent` | Amber-400 | Amber-500 | ⚡ Blitz |
| `full-accent` | Blue-400 | Blue-500 | 📖 Full |
| `progress` | Green-500 | Green-500 | Progress bars |

### Typography

| Token | Size | Weight |
|-------|------|--------|
| `display` | 32px | Black (900) |
| `heading-1` | 24px | Bold (700) |
| `heading-2` | 20px | Bold (700) |
| `heading-3` | 18px | Semibold (600) |
| `body` | 16px | Regular (400) |
| `body-small` | 14px | Regular (400) |
| `caption` | 12px | Medium (500) |

**Font:** Inter or system fonts

### Spacing Scale

| Token | Value |
|-------|-------|
| `space-1` | 4px |
| `space-2` | 8px |
| `space-3` | 12px |
| `space-4` | 16px |
| `space-6` | 24px |
| `space-8` | 32px |

### Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `radius-sm` | 4px | Small elements |
| `radius-md` | 8px | Cards, buttons |
| `radius-lg` | 16px | Modals |
| `radius-full` | 9999px | Circular |

### Animation Timing

| Animation | Duration |
|-----------|----------|
| Player expand/collapse | 300ms |
| Go Deep transition | 200ms |
| Progress bar | 100ms |
| Mode switch | 150ms |
| Hover states | 100ms |
| Page transitions | 200ms |

### Dark Mode

- **Primary design target** (designed dark-first)
- Better for evening/commute listening
- Easier on eyes during long sessions
- More premium, immersive feel

---

## Additional UX Decisions

### Book Deletion

- Removes from owner's library
- Removes from all shared recipients' libraries
- Quota NOT refunded (already consumed)
- Warning shows affected recipients by email
- Downloaded copies remain on recipients' devices

### Quota Visibility

| Quota % | Visibility |
|---------|------------|
| 0-79% | Hidden (Account menu only) |
| 80-99% | Small warning in library + upload |
| 100% | Block generation, show upgrade |

### Offline Behavior

- Downloaded books always play locally
- On connection restore: sync position (background)
- Conflict resolution: most progress wins
- App kill: save position immediately

### Background Audio

| Event | Behavior |
|-------|----------|
| Switch to another app | Continue playing |
| Phone call | Pause → resume after |
| App killed by OS | Save position immediately |
| App resume | Non-blocking sync |

### Download Warnings

| Condition | Warning |
|-----------|---------|
| WiFi + >500MB | "Large download" confirmation |
| Mobile data + >100MB | "Using mobile data" warning |
| Below thresholds | No warning |

### Empty States

| Screen | Behavior |
|--------|----------|
| First open | Show sample books |
| Library empty | "Upload a book" CTA |
| Queue empty | "Browse Library" CTA |
| Shared With Me empty | Hide section |

### Multi-Account

- Downloaded content stays on device after logout
- Downloads bound to account, not accessible by other accounts
- Must share explicitly between accounts

### Notifications

**Push:**
- Audio ready: Yes
- New share: No (email only)
- Removed share: No (email only)

**Email:**
- New share: Yes (configurable)
- Removed share: Yes (configurable)
- Marketing: Opt-in only at signup

### Deep Links

| Pattern | Target |
|---------|--------|
| `/settings` | Settings |
| `/library` | Library |
| `/book/{id}` | Book detail |
| `/book/{id}/chapter/{n}?t=123` | Chapter at timestamp |
| `/share/{token}` | Shared content |

---

## MVP Scope

### In MVP

- PDF upload + two-stage processing
- Blitz Mode (L1/L2) + Full Mode
- Go Deep transition + temporary deep mode
- Audio player (mini, full, lock screen)
- Library with folders + queue
- Offline download + progress sync
- Email-locked sharing + quote-level
- Subscription + quotas
- Dark mode + sample content

### Out of MVP

- Podcast Mode
- EPUB support
- Translation feature
- Non-English UI
- Search in library
- Accessibility features
- Bookmarks/highlights

### If Timeline Pressure, Cut First

1. Folders in library
2. Temporary deep mode checkbox
3. Speed presets (reduce to 1x, 1.5x, 2x)
4. Download size warnings
5. Quote sharing (keep book/chapter only)

---

## Implementation Priorities

### Phase 1: Foundation (Week 1-3)
- Auth (Google OAuth)
- PDF upload + storage
- Processing pipeline (Stage 1)
- Basic library view

### Phase 2: Audio Generation (Week 4-6)
- Audio generation (Stage 2)
- Mini player
- Full player (basic)
- Progress tracking

### Phase 3: Core Experience (Week 7-9)
- Blitz Mode (L1/L2 flow)
- Go Deep transition
- Full player (transcript, buttons, speed)
- Chapter/key point navigation
- Temporary deep mode

### Phase 4: Hands-Free (Week 10-11)
- Lock screen controls
- Background audio
- Speed control
- Queue management
- Offline download

### Phase 5: Sharing (Week 12-13)
- Share creation flow
- Email-locked links
- Recipient landing page
- Quote-level sharing
- Share notifications

### Phase 6: Business & Polish (Week 14-16)
- Subscription/payments
- Quota enforcement
- Settings screen
- Sample content
- Onboarding flow
- Error states
- Dark mode polish

---

## User Testing Plan

### Testing Timeline

| Week | Focus | Users | Key Question |
|------|-------|-------|--------------|
| 3 | PDF Parsing | 10 | Can we parse real PDFs? |
| 6 | Audio Quality | 10-20 | Is audio good enough? |
| 9 | Core UX | 8 | Does depth-on-demand feel magical? |
| 11 | Hands-Free | 5 | Does it work in real life? |
| 13 | Sharing | 8+20 | Will recipients listen? |
| 14-16 | Beta | 20-30 | Will users come back? |

### Success Criteria

| Metric | Target |
|--------|--------|
| PDF parse success | >85% |
| Voice quality | >4.0/5 |
| Go Deep discovery | >60% tap |
| Commute satisfaction | >80% |
| Share open rate | >50% |
| Day 7 retention | >40% |
| NPS | >30 |

---

## Document Status

- **Status:** Complete
- **Version:** 1.0
- **Last Updated:** 2026-04-09
- **Author:** Gotha + Sally (UX Designer)
