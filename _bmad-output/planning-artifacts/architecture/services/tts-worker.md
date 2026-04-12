# TTS Worker Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Python 3.11 |
| **GPU Required** | Optional (Kokoro faster with GPU) |
| **Queue** | NATS JetStream |
| **Output** | AAC 128kbps audio files |
| **Database** | None (single writer pattern) |

---

## Responsibilities

### Primary Functions

1. **Audio Generation** — Convert text to speech
2. **Provider Abstraction** — Support multiple TTS providers
3. **Format Conversion** — Convert to AAC 128kbps
4. **Upload** — Store audio in object storage

### What This Service Does NOT Do

- ❌ Write to database (API Service does this)
- ❌ Decide what to generate (API Service triggers this)
- ❌ Manage playback (Frontend does this)

---

## Queue Interface

### Input: `tts.generate`

```json
{
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "content_type": "l1_summary",
  "key_point_number": null,
  "text": "Text to synthesize...",
  "voice_id": "voice-uuid"
}
```

### Output: `tts.completed`

**Success:**

```json
{
  "status": "success",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "content_type": "l1_summary",
  "key_point_number": null,
  "file_id": "audio-file-uuid",
  "file_path": "audio/book-uuid/audio-file-uuid.aac",
  "duration_ms": 120000,
  "file_size_bytes": 1920000,
  "provider": "kokoro"
}
```

**Failure:**

```json
{
  "status": "failed",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "content_type": "l1_summary",
  "key_point_number": null,
  "error": "TTS provider timeout"
}
```

---

## Processing Pipeline

```
1. Receive job from NATS
         │
         ▼
2. Load voice configuration
         │
         ▼
3. Generate audio (provider)
   ├── Kokoro (local)
   ├── ElevenLabs (API)
   └── Google TTS (API)
         │
         ▼
4. Convert to AAC 128kbps (ffmpeg)
         │
         ▼
5. Upload to S3/R2
         │
         ▼
6. Publish tts.completed
```

---

## Provider Interface

```python
class TTSProvider(ABC):
    @property
    @abstractmethod
    def name(self) -> str:
        pass
    
    @abstractmethod
    async def synthesize(self, request: TTSRequest) -> TTSResult:
        pass
    
    @abstractmethod
    async def list_voices(self, language: str = None) -> list[Voice]:
        pass
    
    @abstractmethod
    async def health_check(self) -> bool:
        pass
    
    @property
    @abstractmethod
    def max_text_length(self) -> int:
        pass
```

---

## Providers

### Kokoro (Development)

| Attribute | Value |
|-----------|-------|
| Type | Local model |
| GPU | Optional (faster with) |
| Cost | Free |
| Quality | Good |
| Max text | 5000 chars |

```python
class KokoroProvider(TTSProvider):
    def __init__(self, model_path: str):
        self.model = kokoro.load_model(model_path)
    
    async def synthesize(self, request: TTSRequest) -> TTSResult:
        audio, sr = self.model.generate(
            text=request.text,
            voice=request.voice.id,
            speed=request.speed
        )
        return TTSResult(audio_data=audio, format="wav", ...)
```

### ElevenLabs (Production)

| Attribute | Value |
|-----------|-------|
| Type | API |
| GPU | N/A |
| Cost | $0.30/1K chars |
| Quality | Excellent |
| Max text | 5000 chars |

```python
class ElevenLabsProvider(TTSProvider):
    async def synthesize(self, request: TTSRequest) -> TTSResult:
        response = await self.client.post(
            f"/text-to-speech/{request.voice.id}",
            json={"text": request.text, ...}
        )
        return TTSResult(audio_data=response.content, format="mp3", ...)
```

### Google TTS (Fallback)

| Attribute | Value |
|-----------|-------|
| Type | API |
| GPU | N/A |
| Cost | $4/1M chars |
| Quality | Very Good |
| Max text | 5000 chars |

---

## Provider Registry

```python
class ProviderRegistry:
    def __init__(self):
        self.providers = {}
        self.primary = None
        self.fallback_chain = []
    
    async def synthesize(self, request: TTSRequest) -> TTSResult:
        for provider_name in [self.primary] + self.fallback_chain:
            provider = self.providers[provider_name]
            try:
                if await provider.health_check():
                    return await provider.synthesize(request)
            except Exception:
                continue
        
        raise RuntimeError("All TTS providers failed")
```

---

## Audio Conversion

```python
async def convert_to_aac(audio_data: bytes, input_format: str) -> bytes:
    """Convert to AAC 128kbps mono using ffmpeg"""
    
    with tempfile.TemporaryDirectory() as tmpdir:
        input_path = f"{tmpdir}/input.{input_format}"
        output_path = f"{tmpdir}/output.aac"
        
        with open(input_path, "wb") as f:
            f.write(audio_data)
        
        ffmpeg.input(input_path).output(
            output_path,
            acodec="aac",
            audio_bitrate="128k",
            ar=44100,
            ac=1  # Mono
        ).run()
        
        with open(output_path, "rb") as f:
            return f.read()
```

---

## Content Types

| Content Type | Description | Typical Duration |
|--------------|-------------|------------------|
| `l1_summary` | Chapter overview | 2-3 minutes |
| `l2_summary` | Key point detail | 30-60 seconds |
| `full` | Full chapter narration | 20-40 minutes |

---

## Voice Configuration

```json
{
  "voice_id": "voice-uuid",
  "provider": "kokoro",
  "provider_voice_id": "af_bella",
  "name": "Bella",
  "language": "en-US",
  "gender": "female",
  "speed": 1.0
}
```

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `NATS_URL` | NATS server | `nats://localhost:4222` |
| `S3_ENDPOINT` | Object storage | required |
| `S3_BUCKET` | Audio bucket | `disona-audio` |
| `TTS_PRIMARY_PROVIDER` | Primary provider | `kokoro` |
| `TTS_FALLBACK_PROVIDERS` | Fallback list | `google` |
| `KOKORO_MODEL_PATH` | Model path | `/models/kokoro-v0_19.pth` |
| `KOKORO_DEVICE` | CPU/CUDA | `cuda` |
| `ELEVENLABS_API_KEY` | API key | (production) |
| `GOOGLE_CREDENTIALS_PATH` | Credentials | (production) |

---

## Worker Configuration

```python
# NATS consumer config
ConsumerConfig(
    durable_name="tts-workers",
    ack_policy=AckPolicy.EXPLICIT,
    max_ack_pending=1  # One job at a time (GPU memory)
)
```

---

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| kokoro | 0.1+ | Local TTS |
| torch | 2.0+ | Kokoro backend |
| ffmpeg-python | 0.2+ | Audio conversion |
| httpx | 0.27+ | API calls |
| nats-py | 2.6+ | NATS client |
| boto3 | 1.34+ | S3 upload |
