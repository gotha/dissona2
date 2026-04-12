# Deployment Specification

## Overview

| Environment | Infrastructure | Orchestration |
|-------------|----------------|---------------|
| **Development** | Local machine | Docker Compose |
| **Production (MVP)** | Single GPU VPS | Docker Compose |
| **Production (Future)** | Split services | K8s / Managed |

---

## MVP: Single VPS + Docker Compose

### Server Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 8 cores | 16 cores |
| RAM | 32 GB | 64 GB |
| GPU | RTX 3080 (10GB) | RTX 4090 (24GB) |
| Storage | 500 GB SSD | 1 TB NVMe |

### Providers

| Provider | GPU Server | Price/month |
|----------|------------|-------------|
| Hetzner | RTX 4000 | ~€150 |
| Vast.ai | RTX 3090 | ~$150 |
| Lambda Labs | RTX 3080 | ~$180 |

---

## Architecture (MVP)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SINGLE GPU VPS                                       │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      Docker Compose                                  │    │
│  │                                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │    │
│  │  │ Traefik  │ │   API    │ │   Auth   │ │ Frontend │              │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘              │    │
│  │                                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐                           │    │
│  │  │   PDF    │ │   LLM    │ │   TTS    │  Workers                  │    │
│  │  │  Worker  │ │  Worker  │ │  Worker  │                           │    │
│  │  └──────────┘ └──────────┘ └──────────┘                           │    │
│  │                                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │    │
│  │  │ Postgres │ │   NATS   │ │  Redis   │ │  Ollama  │ ◄── GPU     │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘              │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │   Cloudflare R2     │
                         │   (Audio Storage)   │
                         └─────────────────────┘
```

---

## Services

| Service | Port | Build |
|---------|------|-------|
| Traefik | 80, 443, 8080 | Image |
| API Service | 8080 | Rust |
| Auth Service | 8081 | Rust |
| Frontend | 3000 | Node/Nginx |
| PDF Worker | - | Python |
| LLM Worker | - | Python |
| TTS Worker | - | Python |
| PostgreSQL | 5432 | Image |
| NATS | 4222 | Image |
| Redis | 6379 | Image |
| MinIO (dev) | 9000 | Image |
| Ollama | 11434 | Image + GPU |

---

## External Services

| Service | Purpose | Environment |
|---------|---------|-------------|
| Cloudflare R2 | Audio storage | Production |
| MinIO | Audio storage | Development |
| Google OAuth | Authentication | Both |
| Let's Encrypt | SSL certificates | Production |

---

## CI/CD

| Stage | Tool | Trigger |
|-------|------|---------|
| Test | GitHub Actions | PR, Push |
| Build | GitHub Actions | Push to main |
| Deploy | SSH + Docker | Push to main |

### Deploy Flow

1. Push to `main` branch
2. GitHub Actions runs tests
3. Build and push Docker images
4. SSH to server
5. `docker compose pull`
6. `docker compose up -d`

---

## Directory Structure

```
disona/
├── docker-compose.yml           # Development
├── docker-compose.prod.yml      # Production overrides
├── services/
│   ├── api/
│   ├── auth/
│   ├── pdf-worker/
│   ├── llm-worker/
│   └── tts-worker/
├── frontend/
├── migrations/
├── scripts/
└── .github/workflows/
```

---

## Scaling Path

| Phase | Users | Infrastructure |
|-------|-------|----------------|
| MVP | <100 | Single VPS + Docker Compose |
| Phase 2 | 100-1000 | Managed DB + Same VPS |
| Phase 3 | 1000-10000 | Split compute + GPU workers |
| Phase 4 | 10000+ | Kubernetes |
