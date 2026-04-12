# Observability Specification

## Overview

**MVP Principle:** Self-hosted, open source tools only. No paid SaaS observability services.

## Grafana Stack

| Component | Purpose | Port | License |
|-----------|---------|------|---------|
| **Grafana** | Dashboards, alerts | 3000 | AGPLv3 |
| **Prometheus** | Metrics storage | 9090 | Apache 2.0 |
| **Loki** | Log aggregation | 3100 | AGPLv3 |
| **Tempo** | Distributed tracing | 4317 | AGPLv3 |
| **Promtail** | Log shipping | - | AGPLv3 |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              SERVICES                                        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐         │
│  │   API    │ │   Auth   │ │   PDF    │ │   LLM    │ │   TTS    │         │
│  │ Service  │ │ Service  │ │  Worker  │ │  Worker  │ │  Worker  │         │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘         │
│       │ metrics    │ logs       │ traces     │            │                 │
└───────┼────────────┼────────────┼────────────┼────────────┼─────────────────┘
        ▼            ▼            ▼            ▼            ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │   Prometheus    │  │    Promtail     │  │  OTLP Collector │             │
│  │  (scrapes)      │  │  (ships logs)   │  │  (traces)       │             │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│           ▼                    ▼                    ▼                       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│  │   Prometheus    │  │      Loki       │  │      Tempo      │             │
│  │   (storage)     │  │   (storage)     │  │   (storage)     │             │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│           └────────────────────┼────────────────────┘                       │
│                                ▼                                            │
│                       ┌─────────────────┐                                   │
│                       │     Grafana     │                                   │
│                       │  (dashboards)   │                                   │
│                       └─────────────────┘                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Service Instrumentation

| Service | Language | Metrics | Logs | Traces |
|---------|----------|---------|------|--------|
| API Service | Rust | prometheus crate | JSON (tracing) | OTLP |
| Auth Service | Rust | prometheus crate | JSON (tracing) | OTLP |
| PDF Worker | Python | prometheus-client | structlog JSON | OTLP |
| LLM Worker | Python | prometheus-client | structlog JSON | OTLP |
| TTS Worker | Python | prometheus-client | structlog JSON | OTLP |

---

## Key Metrics

### API/Auth Services

| Metric | Type | Description |
|--------|------|-------------|
| `http_requests_total` | Counter | Requests by method, endpoint, status |
| `http_request_duration_seconds` | Histogram | Request latency |
| `db_query_duration_seconds` | Histogram | Database query time |

### Workers

| Metric | Type | Description |
|--------|------|-------------|
| `jobs_processed_total` | Counter | Jobs by type, status |
| `job_duration_seconds` | Histogram | Processing time |
| `jobs_in_queue` | Gauge | Current queue depth |
| `llm_tokens_total` | Counter | LLM token usage |
| `tts_audio_generated_seconds` | Counter | Audio duration generated |

---

## Dashboards

| Dashboard | Purpose |
|-----------|---------|
| Service Overview | Requests, errors, latency across services |
| Worker Performance | Job throughput, queue depth, duration |
| Infrastructure | Postgres, NATS, Redis health |
| LLM Usage | Token consumption, costs |
| TTS Usage | Audio hours generated |

---

## Alerting

| Alert | Condition | Severity |
|-------|-----------|----------|
| HighErrorRate | >5% 5xx errors for 5min | Critical |
| ServiceDown | Service unreachable 1min | Critical |
| HighLatency | P99 >1s for 5min | Warning |
| WorkerQueueBackup | Queue >50 for 10min | Warning |
| DiskSpaceLow | <20% free | Warning |

---

## Retention

| Data Type | Retention | Storage |
|-----------|-----------|---------|
| Metrics | 15 days | Prometheus |
| Logs | 7 days | Loki |
| Traces | 7 days | Tempo |

---

## Future Considerations

When scaling beyond MVP, consider:
- Grafana Cloud (managed)
- Mimir (scalable metrics)
- Distributed Loki/Tempo
