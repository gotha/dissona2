# Business Cases

## Overview

This document explains the business rationale behind each feature and service in Disona.

---

## Core Value Proposition

**Problem:** Professionals want to learn from books but lack time. Existing audiobooks are:
- Long (8-15 hours)
- All-or-nothing (can't skim)
- No depth control (can't dive deeper)

**Solution:** Disona offers "Depth on Demand" — users control how deep they go:
- **Blitz Mode:** 30-minute overview of a book
- **Go Deep:** Seamlessly expand any topic
- **Full Mode:** Complete audiobook experience

---

## Feature Business Cases

### 1. Two-Stage Processing (Analysis → Generation)

**What:** Analysis happens automatically; audio generation is user-triggered.

**Why it matters:**

| Stakeholder | Benefit |
|-------------|---------|
| **User** | Can preview chapter structure before committing quota |
| **Business** | Reduces compute waste on abandoned uploads |
| **Operations** | GPU resources used only when needed |

**Cost impact:**
- LLM costs for analysis: ~$0.05 per book
- TTS costs for generation: ~$2-5 per book
- Separating stages saves ~30% on abandoned books

---

### 2. Depth on Demand (Blitz ↔ Full)

**What:** Users can seamlessly switch between summary and full content.

**Why it matters:**

| User Need | How Disona Solves It |
|-----------|---------------------|
| "I have 20 minutes" | Blitz Mode covers entire chapter |
| "That point was interesting" | Go Deep expands just that topic |
| "I want the full experience" | Full Mode plays complete narration |

**Competitive advantage:**
- Audible: No skimming option
- Blinkist: No depth option
- Disona: Best of both worlds

---

### 3. Progressive Audio Availability

**What:** Chapters become playable as soon as generated, not waiting for entire book.

**Why it matters:**

| Without | With |
|---------|------|
| Wait 30 min for book to finish | Start listening in 60 seconds |
| User leaves, forgets about book | User engaged immediately |
| High abandonment | Low abandonment |

**Implementation:** TTS worker processes chapter by chapter, API publishes availability per chapter.

---

### 4. Offline-First Design

**What:** Downloaded books work fully offline, including Go Deep.

**Why it matters:**

| Use Case | Requirement |
|----------|-------------|
| Subway commute | No internet |
| International travel | Roaming costs |
| Rural areas | Poor connectivity |

**Business impact:**
- Increases daily usage (commute is prime time)
- Reduces perceived "app is broken" complaints
- Matches user expectations from Spotify/Audible

---

### 5. Email-Locked Sharing

**What:** Shared content requires email verification to access.

**Why it matters:**

| Aspect | Benefit |
|--------|---------|
| **Growth** | Viral loop: share → sign up → share |
| **Anti-piracy** | Can't share public links on forums |
| **Tracking** | Know who accessed what |
| **Conversion** | Shared content is a teaser |

**Conversion funnel:**
1. User shares chapter (requires subscription)
2. Recipient verifies email (lead captured)
3. Recipient listens (experiences product)
4. Recipient sees "Upload your own books" CTA

---

### 6. Chapter Length Normalization

**What:** Chapters automatically split/merged to 30-45 minute targets.

**Why it matters:**

| Problem | Solution |
|---------|----------|
| 2-hour chapter is overwhelming | Split into 3x 40-minute parts |
| 3-minute chapter feels incomplete | Merge with adjacent chapters |
| Inconsistent UX | Predictable chapter lengths |

**User psychology:**
- "One chapter" feels achievable
- Consistent progress = satisfaction
- Matches podcast episode lengths

---

## Service Business Cases

### API Service (Rust)

**Why it exists:**
- Central business logic owner
- Single database writer (data consistency)
- High-performance REST API

**Why Rust:**
- Low latency for real-time features
- Memory safety without garbage collection
- Efficient for I/O-bound workloads

**Business value:**
- Handles 500+ concurrent users on minimal resources
- Reduces cloud costs vs. interpreted languages
- Type safety reduces production bugs

---

### Auth Service (Rust)

**Why separate from API Service:**
- Security boundary (isolated credentials)
- Can scale independently (auth is called on every request)
- Forward auth pattern with Traefik

**Business value:**
- Google OAuth reduces sign-up friction
- JWT enables stateless scaling
- Rate limiting protects against abuse

---

### PDF Worker (Python)

**Why Python:**
- Best PDF libraries (PyMuPDF, Tesseract bindings)
- ML ecosystem for embeddings
- Mature text processing tools

**Why separate service:**
- CPU-intensive (OCR)
- Different scaling needs
- Isolates failures (bad PDF doesn't crash API)

**Business value:**
- Handles 90%+ of PDF formats
- Quality detection warns users early
- Chapter detection reduces manual work

---

### LLM Worker (Python)

**Why Python:**
- LiteLLM integration
- ML ecosystem
- Prompt engineering tools

**Why separate service:**
- Different cost profile (LLM calls are expensive)
- Rate limiting per provider
- Easy to swap providers

**Business value:**
- Generates summaries at scale
- Provider abstraction reduces vendor lock-in
- Quality prompts = better user experience

---

### TTS Worker (Python)

**Why Python:**
- TTS library integrations
- Audio processing tools
- GPU framework compatibility

**Why separate service:**
- GPU resource management
- Sequential processing (avoid OOM)
- Provider abstraction

**Business value:**
- High-quality audio = premium feel
- Provider flexibility = cost optimization
- Local dev = fast iteration

---

## Infrastructure Business Cases

### NATS JetStream (vs Kafka/RabbitMQ)

**Why NATS:**
- Simple to operate (single binary)
- Pull-based consumers (GPU-friendly)
- Lightweight (low resource overhead)

**Business value:**
- Runs on single machine (dev)
- Scales to production
- Lower ops burden for small team

---

### Ollama (vs direct API calls)

**Why Ollama:**
- Local development (no API costs)
- GPU consolidation (LLM + embeddings)
- OpenAI-compatible API

**Business value:**
- $0 LLM cost during development
- Faster iteration (no rate limits)
- Easy production migration (LiteLLM)

---

### Cloudflare R2 (vs S3/GCS)

**Why R2:**
- Zero egress fees
- S3-compatible API
- Global CDN included

**Business value:**
- Audio streaming is egress-heavy
- Saves $8,000+/month at scale
- Same API as MinIO (dev/prod parity)
