---
title: "Product Brief: Disona"
status: "final"
created: "2026-04-07"
updated: "2026-04-07"
inputs: [user-discovery-session, competitive-research]
---

# Product Brief: Disona

## Executive Summary

**Disona** is a mobile-first personal research assistant that transforms any written content — books, articles, research papers — into intelligent audio experiences. Users upload what they want to learn, and Disona gives them three powerful ways to consume it: full audiobook narration, rapid "blitz mode" summaries with depth-on-demand, or AI-generated multi-host podcast discussions.

The core insight: people want to learn while their hands are busy — commuting, exercising, doing chores. Today's options force a choice: either consume pre-made content (Blinkist, Audible) or use desktop-bound research tools (NotebookLM). Disona breaks this tradeoff. You create your own audio learning content from your own sources, then consume it anywhere your ears are free.

This is not another book summary app. It's a personal audio learning studio that travels with you.

## The Problem

**Knowledge workers and students are drowning in reading they can't get to.**

The stack of articles, PDFs, and books grows faster than the time to read them. Meanwhile, hours of "dead time" pass each week — commutes, workouts, walks — when hands and eyes are occupied but ears are free.

Today's solutions force painful tradeoffs:

| Current Option | The Problem |
|----------------|-------------|
| **Blinkist** | Someone else decides what's important. Summaries are generic, quality varies. You consume their catalog, not your sources. |
| **NotebookLM** | Powerful synthesis, but desktop-only. Mobile app is crippled — no sharing, no export, rough UX. Can't use it on a commute. |
| **Audible/Audiobooks** | Professional narration, but only for published books. Expensive. No summarization. No depth control. |
| **Speechify/TTS** | Just reads text aloud. No intelligence — no chapter detection, no summarization, no multi-source synthesis. |

The result: valuable content sits unread. Learning happens in fragments. People feel perpetually behind.

## The Solution

Disona lets you **upload anything you want to learn** — a PDF, a book, multiple articles — and generates intelligent audio in three modes:

### 📖 Audiobook Mode
Full narration of your content, intelligently chunked into chapter episodes. Listen sequentially, like a professional audiobook — but created instantly from any source.

### ⚡ Blitz Mode
~1 minute summary per chapter, plus key point breakdowns. Quickly grasp the structure and substance of an entire book in 15 minutes. When something catches your interest, tap to hear the full chapter as written. **Depth on demand.**

### 🎙️ Podcast Mode
Upload your sources; Disona identifies the core concepts and generates a multi-host (2-4 voices) discussion — complete with debate, different perspectives, and natural conversation. Learning through discourse, not monologue.

**Key UX commitments:**
- Resume from exact second (critical for commute use)
- Offline download (listen anywhere)
- Private sharing with friends (link or in-app)

## What Makes This Different

| Dimension | Disona | Alternatives |
|-----------|--------|--------------|
| **Content source** | Your sources | Their catalog |
| **Depth control** | Skim → deep dive on demand | One-size-fits-all |
| **Platform** | Mobile-first PWA | Desktop-first or crippled mobile |
| **Synthesis** | Multi-source, multi-host | Single-source or flat narration |
| **Social** | Share what you create | Consume in isolation |

**The moat:** Disona is creator-first. You're not consuming what someone else summarized — you're building your own learning assets from sources that matter to you. And when you create something valuable, you can share it with people you care about.

## Who This Serves

**Primary:** Students and professionals who want to learn more about specific topics while their hands are occupied — commuting, training, doing chores.

They're not looking for pre-packaged summaries of bestsellers. They have specific things they need to understand — papers for a project, books for professional development, articles they've been meaning to read. They want to create learning content from their own stack.

**Secondary:** Registered free users who receive shared content. They listen but don't generate — a growth loop and conversion path.

## Success Criteria

**Year 1 target:** 2,000 actively paying subscribers/month

**User success signals:**
- Time-to-first-listen under 5 minutes from upload
- Weekly active listening (habitual use on commute/exercise)
- Shares per creator (viral coefficient)
- Retention: users who pay month 2, month 3

**Aha moments we're designing for:**
1. "I understand this 300-page book in 15 minutes" — the structure clicks, they know what to deep-dive
2. "My friend loved what I shared" — social validation, offline discussion sparked

## Scope

### MVP (V1)
- ✅ PWA (Progressive Web App) — mobile-first, web-based
- ✅ All three modes: Audiobook, Blitz, Podcast
- ✅ PDF upload with automatic chapter detection
- ✅ Playlist/queue feature — batch multiple uploads into continuous listening sessions
- ✅ Resume from exact second
- ✅ Private sharing (in-app users + email invite)
- ✅ Two subscription tiers: €5/20hrs, €20/100hrs
- ✅ Free tier: listen to shared content only

### Explicitly NOT in V1
- ❌ Native mobile apps (iOS/Android)
- ❌ Public content discovery/library
- ❌ Creator monetization features
- ❌ Live collaboration
- ❌ Third-party integrations

### Known Risks & Mitigations

**Chapter detection reliability:** Automatic chapter detection from PDFs varies in quality depending on source formatting and OCR accuracy. Mitigation: implement semantic chunking as fallback — analyze text structure and create logical segments even when explicit chapter markers are absent. This is a solvable engineering problem, not a blocker.

### Open Questions (Requires Research)

**Go-to-market strategy:** How do we acquire the first 100 paying users? Potential channels to explore:
- Student communities (Reddit, Discord study groups)
- Product Hunt launch
- Productivity/learning influencers
- Content creator partnerships
- Targeted ads to "commute learners"

This requires a dedicated GTM planning exercise before launch.

## Vision

If Disona succeeds, it becomes **the platform where people turn knowledge into audio — for themselves and for others.**

Year 1: Personal learning tool with private sharing.
Year 2-3: Enable creators to monetize their audio. Disona becomes a creator economy for educational audio — not pre-published audiobooks, but custom learning content generated by people who deeply understand their domains.

The sharing mechanic isn't just a feature — it's the growth engine. Every shared audiobook or podcast is a marketing touchpoint. Every "thank you" from a friend is a conversion opportunity.
