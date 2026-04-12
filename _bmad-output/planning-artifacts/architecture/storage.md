# Storage Specification

## Overview

| Environment | Technology | Purpose |
|-------------|------------|---------|
| Development | MinIO | S3-compatible local storage |
| Production | Cloudflare R2 | S3-compatible, zero egress |

---

## Why Cloudflare R2

| Aspect | S3 | R2 |
|--------|----|----|
| Egress cost (100TB) | ~$8,500/mo | $0/mo |
| Storage cost | $0.023/GB | $0.015/GB |
| S3 compatibility | Native | Full |
| CDN | CloudFront extra | Included |

**Audio streaming is egress-heavy** — R2 saves significant costs.

---

## Buckets

### disona-uploads

**Purpose:** Temporary storage for uploaded PDFs

| Setting | Value |
|---------|-------|
| Access | Private |
| Lifecycle | Delete after 7 days |
| Max object size | 100 MB |

**Structure:**

```
disona-uploads/
└── {book_uuid}/
    └── original.pdf
```

### disona-audio

**Purpose:** Generated audio files

| Setting | Value |
|---------|-------|
| Access | Public (via CDN) |
| Lifecycle | No expiration |
| Cache-Control | `public, max-age=31536000, immutable` |

**Structure:**

```
disona-audio/
└── audio/
    └── {book_uuid}/
        ├── manifest.json
        ├── {file_uuid}.aac    (L1 summary)
        ├── {file_uuid}.aac    (L2 summary)
        └── {file_uuid}.aac    (Full chapter)
```

---

## URL Structure

### Upload URLs (Private, Signed)

```
# Upload: Presigned PUT URL
https://s3.disona.app/disona-uploads/{book_uuid}/original.pdf
  ?X-Amz-Credential=...
  &X-Amz-Signature=...
  &X-Amz-Expires=3600
```

### Audio URLs (Public, UUID-based)

```
# No signing needed — UUID is unguessable
https://cdn.disona.app/audio/{book_uuid}/{file_uuid}.aac
```

**Security model:**
- UUIDs are unguessable (122 bits of entropy)
- No authentication at CDN level
- Simpler caching, no token expiry issues

---

## Audio File Naming

```
File ID: UUID v4
Path: audio/{book_uuid}/{file_uuid}.aac

Example:
audio/a1b2c3d4-e5f6-7890-abcd-ef1234567890/
    ├── 9f8e7d6c-5b4a-3210-fedc-ba0987654321.aac
    ├── 1a2b3c4d-5e6f-7890-1234-567890abcdef.aac
    └── 2b3c4d5e-6f78-9012-3456-7890abcdef12.aac
```

---

## Manifest Structure

```json
{
  "book_id": "a1b2c3d4-...",
  "title": "The Manager's Path",
  "total_duration_ms": 16200000,
  "format": "aac",
  "bitrate": 128,
  
  "chapters": [
    {
      "chapter_id": "ch-uuid",
      "number": 1,
      "title": "Management 101",
      
      "l1_summary": {
        "file_id": "9f8e7d6c-...",
        "duration_ms": 120000
      },
      
      "l2_summaries": [
        {
          "key_point": 1,
          "file_id": "1a2b3c4d-...",
          "duration_ms": 45000,
          "full_chapter_start_ms": 0,
          "full_chapter_end_ms": 180000
        }
      ],
      
      "full_chapter": {
        "file_id": "2b3c4d5e-...",
        "duration_ms": 1200000
      }
    }
  ]
}
```

---

## CDN Configuration

### Cloudflare R2 + CDN

```
┌─────────────────────────────────────────────────────────────┐
│                        Cloudflare                            │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  R2 Bucket: disona-audio                               │  │
│  │  - Public access enabled                               │  │
│  │  - Custom domain: cdn.disona.app                      │  │
│  └───────────────────────────────────────────────────────┘  │
│                          │                                   │
│                          ▼                                   │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  CDN Edge                                              │  │
│  │  - Global edge caching                                 │  │
│  │  - Cache-Control respected                            │  │
│  │  - Range requests supported                           │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Cache Headers

```http
Cache-Control: public, max-age=31536000, immutable
Content-Type: audio/aac
Accept-Ranges: bytes
```

---

## Development (MinIO)

### Docker Compose

```yaml
minio:
  image: minio/minio
  command: server /data --console-address ":9001"
  ports:
    - "9000:9000"
    - "9001:9001"
  environment:
    MINIO_ROOT_USER: minioadmin
    MINIO_ROOT_PASSWORD: minioadmin
  volumes:
    - minio-data:/data
```

### Create Buckets

```bash
mc alias set local http://localhost:9000 minioadmin minioadmin
mc mb local/disona-uploads
mc mb local/disona-audio
mc anonymous set download local/disona-audio
```

---

## Size Estimates

### Per Book

| Content | Count | Size Each | Total |
|---------|-------|-----------|-------|
| L1 summaries | 15 | 2 MB | 30 MB |
| L2 summaries | 60 | 0.8 MB | 48 MB |
| Full chapters | 15 | 25 MB | 375 MB |
| **Total** | | | **~450 MB** |

### Platform Scale

| Users | Books | Storage | Monthly Egress |
|-------|-------|---------|----------------|
| 1,000 | 5,000 | 2.25 TB | 10 TB |
| 10,000 | 50,000 | 22.5 TB | 100 TB |
| 100,000 | 500,000 | 225 TB | 1 PB |

---

## Backup Strategy

| Bucket | Backup | Retention |
|--------|--------|-----------|
| disona-uploads | None (temporary) | 7 days |
| disona-audio | Daily snapshot | 30 days |

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `S3_ENDPOINT` | S3 endpoint | `http://minio:9000` |
| `S3_ACCESS_KEY` | Access key | required |
| `S3_SECRET_KEY` | Secret key | required |
| `S3_BUCKET_UPLOADS` | Upload bucket | `disona-uploads` |
| `S3_BUCKET_AUDIO` | Audio bucket | `disona-audio` |
| `CDN_BASE_URL` | CDN URL | `https://cdn.disona.app` |
