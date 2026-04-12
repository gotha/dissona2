# NATS JetStream Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Technology** | NATS JetStream |
| **Purpose** | Async job queue, event streaming |
| **Persistence** | File-based |
| **Port** | 4222 |

---

## Why NATS JetStream

| Requirement | NATS Capability |
|-------------|-----------------|
| Pull-based consumers | ✅ Workers pull when ready |
| GPU job control | ✅ `max_ack_pending=1` |
| Persistence | ✅ JetStream storage |
| Simple operations | ✅ Single binary |
| Event streaming | ✅ Fan-out consumers |

---

## Streams

### JOBS Stream

**Purpose:** Job queue for workers

```
Name: JOBS
Subjects: pdf.parse, llm.*, tts.generate
Retention: WorkQueue (delete after ack)
Storage: File
Replicas: 1 (dev) / 3 (prod)
MaxAge: 7 days
```

### EVENTS Stream

**Purpose:** Completion events, fan-out

```
Name: EVENTS
Subjects: pdf.completed, llm.*.completed, tts.completed
Retention: Limits
Storage: File
Replicas: 1 (dev) / 3 (prod)
MaxAge: 24 hours
MaxMsgs: 100000
```

---

## Subjects

### Job Subjects (Input to Workers)

| Subject | Consumer | Purpose |
|---------|----------|---------|
| `pdf.parse` | PDF Worker | Parse uploaded PDF |
| `llm.segment` | LLM Worker | Segment chapter into key points |
| `llm.summarize.l1` | LLM Worker | Generate L1 summary |
| `llm.summarize.l2` | LLM Worker | Generate L2 summary |
| `tts.generate` | TTS Worker | Generate audio |

### Event Subjects (Output from Workers)

| Subject | Publisher | Purpose |
|---------|-----------|---------|
| `pdf.completed` | PDF Worker | PDF processing done |
| `llm.segment.completed` | LLM Worker | Segmentation done |
| `llm.summarize.completed` | LLM Worker | Summary done |
| `tts.completed` | TTS Worker | Audio generation done |

---

## Consumers

### PDF Worker Consumer

```
Name: pdf-workers
Stream: JOBS
Filter: pdf.parse
AckPolicy: Explicit
MaxAckPending: 1
DeliverPolicy: All
AckWait: 10 minutes
MaxDeliver: 3
```

### LLM Worker Consumer

```
Name: llm-segment-workers
Stream: JOBS
Filter: llm.segment
AckPolicy: Explicit
MaxAckPending: 2
DeliverPolicy: All
AckWait: 5 minutes
MaxDeliver: 3
```

### TTS Worker Consumer

```
Name: tts-workers
Stream: JOBS
Filter: tts.generate
AckPolicy: Explicit
MaxAckPending: 1  # One at a time (GPU)
DeliverPolicy: All
AckWait: 10 minutes
MaxDeliver: 3
```

### API Service Event Consumers

```
Name: api-pdf-handler
Stream: EVENTS
Filter: pdf.completed
AckPolicy: Explicit

Name: api-llm-handler
Stream: EVENTS
Filter: llm.*.completed
AckPolicy: Explicit

Name: api-tts-handler
Stream: EVENTS
Filter: tts.completed
AckPolicy: Explicit
```

---

## Message Formats

### Job Message

```json
{
  "job_id": "uuid",
  "book_id": "uuid",
  "...": "job-specific fields"
}
```

### Event Message

```json
{
  "status": "success" | "failed",
  "job_id": "uuid",
  "book_id": "uuid",
  "...": "result fields",
  "error": "if failed"
}
```

---

## Retry Behavior

| Scenario | Behavior |
|----------|----------|
| Worker crash | Message redelivered after AckWait |
| Explicit NAK | Message redelivered immediately |
| Max retries | Message sent to dead letter |
| Worker restart | Continues from last ack |

---

## Dead Letter Handling

```
Stream: JOBS_DLQ
Subjects: *.dlq
Retention: Limits
MaxAge: 30 days
```

After `MaxDeliver` attempts:
1. Message moved to DLQ
2. Alert sent to ops
3. Manual investigation required

---

## Configuration

### Development (docker-compose.yml)

```yaml
nats:
  image: nats:latest
  command:
    - "--jetstream"
    - "--store_dir=/data"
  ports:
    - "4222:4222"
    - "8222:8222"  # Monitoring
  volumes:
    - nats-data:/data
```

### Production

```yaml
nats:
  image: nats:latest
  command:
    - "--jetstream"
    - "--store_dir=/data"
    - "--cluster_name=disona"
    - "--cluster=nats://0.0.0.0:6222"
  deploy:
    replicas: 3
```

---

## Monitoring

### HTTP Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /healthz` | Health check |
| `GET /varz` | Server variables |
| `GET /jsz` | JetStream stats |
| `GET /connz` | Connections |

### Key Metrics

| Metric | Alert Threshold |
|--------|-----------------|
| Consumer pending | > 1000 messages |
| Consumer lag | > 5 minutes |
| Stream size | > 10 GB |
| Redelivery rate | > 10% |

---

## Client Configuration

### Rust (async-nats)

```rust
let client = async_nats::connect("nats://nats:4222").await?;
let js = async_nats::jetstream::new(client);

let consumer = js.get_stream("JOBS").await?
    .get_or_create_consumer("pdf-workers", Config {...}).await?;
```

### Python (nats-py)

```python
nc = await nats.connect("nats://nats:4222")
js = nc.jetstream()

psub = await js.pull_subscribe(
    "pdf.parse",
    durable="pdf-workers",
    stream="JOBS"
)
```
