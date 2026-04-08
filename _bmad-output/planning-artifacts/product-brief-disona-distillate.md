---
title: "Product Brief Distillate: Disona"
type: llm-distillate
source: "product-brief-disona.md"
created: "2026-04-07"
purpose: "Token-efficient context for downstream PRD creation"
---

# Disona — Detail Pack for PRD Creation

## Core Concept (Dense Summary)

- Mobile-first PWA for converting uploaded content (PDFs, books, articles) into intelligent audio
- Three consumption modes: Audiobook (full narration), Blitz (chapter summaries + depth-on-demand), Podcast (AI multi-host discussion)
- Creator-first model: users generate content from their own sources, not pre-made catalog
- Private sharing only — no public discovery, friends-only distribution
- Target: students + professionals learning while hands are busy (commute, exercise, chores)

## User Scenarios (Detailed)

- **Commute learner**: Uploads 300-page PDF on Sunday, listens in Blitz mode during Monday commute, identifies 3 chapters worth deep-diving, switches to full narration for those chapters over the week
- **Student cramming**: Uploads multiple research papers on same topic, generates podcast-style debate to hear different angles before writing thesis
- **Professional development**: Uploads industry book, shares generated audiobook with colleague, they discuss insights over lunch
- **Article backlog clearer**: Drags 10 saved articles into queue, listens through summaries during gym session, stars 2 for full listen later

## Aha Moments (Design For These)

1. **"I understand this book in minutes"** — Upload big content → see structure instantly → know what's worth deep-diving vs skimming
2. **"My friend loved what I shared"** — Share creation → friend thanks them → offline discussion sparked → social validation

## Technical Preferences & Constraints

- **Platform**: PWA first, no native apps in V1
- **Audio player requirements**: 
  - Resume from exact second (CRITICAL — commute use case depends on this)
  - Offline download capability
  - Queue/playlist for continuous listening sessions
- **Chapter detection**: Primary via PDF structure, fallback to semantic chunking when markers absent
- **Content input**: PDF upload initially, multi-source synthesis (combine articles/papers)

## Pricing Model (Specific)

| Tier | Price | Allocation | Notes |
|------|-------|------------|-------|
| Free | €0 | Listen to shared content only | No generation, conversion funnel |
| Basic | €5/month | 20 hours audio generation | Entry tier |
| Pro | €20/month | 100 hours audio generation | Power users |

- Generated audio owned by user forever — download, keep, share
- No storage fees, no expiration
- Hours = generation time, not listening time

## Sharing Mechanics (Detailed)

- Share with specific individuals only (not public)
- Two methods: in-app user share, email invite (recipient must create account)
- Free users can receive and listen to shared content
- Sharing is growth lever — every share is marketing touchpoint
- Creator retains full ownership of generated audio
- User responsible for source content copyright compliance

## Competitive Intelligence (Preserve for Positioning)

### Blinkist Weaknesses
- "Summaries are low quality — don't match my sense of the book" (HN user)
- "Reading a 15-minute summary doesn't make you 'read' a book"
- High churn: users read 5-7 summaries then stop
- Content created by others with their own opinions injected
- No depth control — take it or leave it

### NotebookLM Weaknesses  
- Mobile app is "rough" — missing mind maps, reports, source exclusion
- "No export — insights stay trapped in Google's ecosystem"
- Can't share notebooks from mobile app
- Syncing delays between devices
- Desktop-first design doesn't serve commute use case

### Speechify/TTS Tools
- Pure narration — no intelligence, no summarization
- No chapter detection or structure awareness
- No multi-source synthesis
- No podcast generation

## Rejected/Deferred Ideas (Don't Re-Propose)

| Idea | Status | Rationale |
|------|--------|-----------|
| Creator monetization | Deferred to Year 2-3 | Focus on core value first, monetization adds complexity |
| Native iOS/Android apps | Deferred | PWA sufficient for MVP, reduces dev scope |
| Public content library | Rejected for V1 | Privacy-first positioning, avoids copyright complexity |
| Live collaboration | Out of scope | Not core to "personal learning" positioning |
| Third-party integrations | Deferred | Readwise/Pocket integration attractive but not MVP |
| Browser extension | Not discussed | Potential future friction reducer for content capture |

## Success Metrics (Specific)

- **Year 1 target**: 2,000 actively paying subscribers/month
- **Activation**: Time-to-first-listen under 5 minutes from upload
- **Engagement**: Weekly active listening (habitual use)
- **Virality**: Shares per creator (growth loop health)
- **Retention**: Month 2, Month 3 payment retention rates

## Open Questions (Unresolved)

1. **GTM strategy**: How to acquire first 100 users? Channels to explore:
   - Student communities (Reddit, Discord)
   - Product Hunt launch
   - Productivity influencers
   - Learning-focused content creators
   - Targeted ads ("commute learners")
   
2. **Product naming**: "Disona" is codename — final name TBD (workshop later)

3. **Unit economics validation**: €5/20hrs assumed viable — validate against actual TTS/LLM costs

## Technical Risks (Flagged)

- **Chapter detection reliability**: OCR quality varies, PDF formatting inconsistent
  - Mitigation: Semantic chunking fallback, create logical segments from text structure
  - Severity: Medium — solvable engineering problem, not blocker

- **Audio generation quality**: LLM podcast mode could hallucinate or misrepresent source
  - Mitigation: Quality checks, user preview before sharing
  - Severity: Medium — affects trust if shared content is inaccurate
