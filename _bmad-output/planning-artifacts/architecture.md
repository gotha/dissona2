---
stepsCompleted: ["step-01-init"]
inputDocuments:
  - "_bmad-output/planning-artifacts/prd.md"
  - "_bmad-output/planning-artifacts/ux-design.md"
  - "_bmad-output/planning-artifacts/product-brief-disona.md"
workflowType: 'architecture'
project_name: 'Disona'
user_name: 'Gotha'
date: '2026-04-09'
status: draft
---

# Architecture Decision Document — Disona

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

---

## Project Overview

**Disona** transforms written content (PDFs) into intelligent audio learning experiences with "depth on demand" — users can skim summaries (Blitz Mode), listen to full narration (Full Mode), or seamlessly dive deeper based on interest.

### Key Technical Challenges

1. **PDF Parsing** — Extract structure from varied PDF formats
2. **Two-Stage Processing** — Analysis (auto) → Audio generation (on-demand)
3. **TTS Integration** — High-quality neural voices, <90 second latency for first chapter
4. **PWA Platform** — Mobile-first with offline support, lock screen controls
5. **Real-time Sync** — Cross-device progress, conflict resolution
6. **Email-Locked Sharing** — Secure, personal sharing mechanism

### Platform Decisions (from UX Spec)

| Decision | Choice |
|----------|--------|
| Platform | Mobile-first PWA |
| Design System | Tailwind + Radix + Framer Motion |
| Dark Mode | Primary (designed dark-first) |
| Offline | Full support required |
| Lock Screen | MediaSession API |

---

## System Architecture

### Services

| Service | Language | Purpose |
|---------|----------|---------|
| API Service | Rust/Actix | Main API, database writer |
| Auth Service | Rust/Actix | OAuth, JWT issuance |
| PDF Worker | Python | Document parsing, chapter detection |
| LLM Worker | Python | Summarization, segmentation |
| TTS Worker | Python | Text-to-speech generation |
| Frontend | React/Vite | Mobile-first PWA |

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| API Language | Rust | Performance, type safety |
| Worker Language | Python | ML ecosystem, LiteLLM |
| Message Queue | NATS JetStream | Lightweight, durable |
| Database | PostgreSQL (2 instances) | Auth/API isolation |
| Storage | S3-compatible (MinIO) | Scalable, CDN-ready |
| Auth | JWT shared secret | Stateless validation |

---

## Detailed Specifications

See subdirectory `./architecture/` for detailed specs:

### Infrastructure
- [Database](./architecture/database.md) - PostgreSQL schema
- [Database Diagram](./architecture/database-diagram.md) - ER diagram
- [NATS](./architecture/nats.md) - Event streaming
- [Storage](./architecture/storage.md) - S3/MinIO object storage
- [LLM](./architecture/llm.md) - LiteLLM proxy
- [Traefik](./architecture/traefik.md) - API gateway
- [Deployment](./architecture/deployment.md) - Docker, environments
- [Observability](./architecture/observability.md) - Logging, metrics, tracing
- [Security](./architecture/security.md) - Auth, encryption
- [Offline Sync](./architecture/offline-sync.md) - PWA sync strategy

### Services
- [API Service](./architecture/services/api-service.md)
- [Auth Service](./architecture/services/auth-service.md)
- [PDF Worker](./architecture/services/pdf-worker.md)
- [LLM Worker](./architecture/services/llm-worker.md)
- [TTS Worker](./architecture/services/tts-worker.md)

### Frontend
- [Frontend Architecture](./architecture/frontend/architecture.md)

