# LLM Worker Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Python 3.11 |
| **GPU Required** | No (uses Ollama via HTTP) |
| **Queue** | NATS JetStream |
| **LLM Client** | LiteLLM |
| **Database** | None (single writer pattern) |

---

## Responsibilities

### Primary Functions

1. **Chapter Segmentation** — Break chapters into key points
2. **L1 Summarization** — Generate chapter overview (2-3 paragraphs)
3. **L2 Summarization** — Generate key point details (1-2 paragraphs)

### What This Service Does NOT Do

- ❌ Write to database (API Service does this)
- ❌ Run LLM locally (Ollama does this)
- ❌ Generate audio (TTS Worker does this)

---

## Queue Interface

### Job Types

| Subject | Input | Output |
|---------|-------|--------|
| `llm.segment` | Chapter text | Key points list |
| `llm.summarize.l1` | Chapter + key points | L1 summary text |
| `llm.summarize.l2` | Key point text | L2 summary text |

---

### Input: `llm.segment`

```json
{
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "chapter_number": 1,
  "chapter_title": "Management 101",
  "chapter_text": "Full chapter text..."
}
```

### Output: `llm.segment.completed`

```json
{
  "status": "success",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "chapter_number": 1,
  "key_points": [
    {
      "number": 1,
      "title": "What is Management?",
      "summary": "One sentence summary...",
      "text": "Full text for this key point..."
    }
  ],
  "usage": {
    "prompt_tokens": 1500,
    "completion_tokens": 500,
    "model": "qwen2.5:14b"
  }
}
```

---

### Input: `llm.summarize.l1`

```json
{
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "chapter_title": "Management 101",
  "chapter_text": "Full chapter text...",
  "key_points": [
    {"number": 1, "title": "...", "summary": "..."},
    {"number": 2, "title": "...", "summary": "..."}
  ]
}
```

### Output: `llm.summarize.completed`

```json
{
  "status": "success",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "content_type": "l1_summary",
  "summary_text": "This chapter explores the fundamentals of management...",
  "usage": {
    "prompt_tokens": 2000,
    "completion_tokens": 300,
    "model": "qwen2.5:14b"
  }
}
```

---

### Input: `llm.summarize.l2`

```json
{
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "chapter_title": "Management 101",
  "chapter_theme": "Fundamentals of management",
  "key_point_number": 1,
  "key_point_title": "What is Management?",
  "key_point_text": "Text for this key point..."
}
```

### Output: `llm.summarize.completed`

```json
{
  "status": "success",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "chapter_id": "chapter-uuid",
  "content_type": "l2_summary",
  "key_point_number": 1,
  "summary_text": "Management is fundamentally about...",
  "usage": {...}
}
```

---

## Prompts

### Segmentation Prompt

```
You are an expert at analyzing non-fiction books.

Given a chapter, identify 3-7 key points:
1. Main concepts/ideas
2. Start and end positions in text

Output JSON:
{
  "chapter_theme": "...",
  "key_points": [
    {"number": 1, "title": "...", "summary": "...", "text_start": "...", "text_end": "..."}
  ]
}
```

### L1 Summary Prompt

```
Create a 2-3 paragraph overview of this chapter.
- Capture main theme and ideas
- Write in flowing prose for audio narration
- No bullet points or lists
```

### L2 Summary Prompt

```
Create a 1-2 paragraph summary of this key point.
- Explain the concept clearly
- Include important details
- Self-contained (works without context)
- Suitable for audio narration
```

---

## Provider Configuration

### Development (Ollama)

```python
model = "ollama/qwen2.5:14b"
api_base = "http://ollama:11434"
```

### Production (OpenAI)

```python
model = "gpt-4o-mini"
api_key = os.environ["OPENAI_API_KEY"]
```

### LiteLLM Usage

```python
import litellm

response = await litellm.acompletion(
    model=model,
    messages=messages,
    temperature=0.3,
    max_tokens=2000,
    api_base=api_base  # For Ollama
)
```

---

## Error Handling

| Error | Action |
|-------|--------|
| LLM timeout | Retry up to 3 times |
| Rate limit | Exponential backoff |
| Invalid response | Retry with different prompt |
| Context too long | Truncate input |
| All retries failed | Publish failure event |

---

## Token Estimation

| Job Type | Input Tokens | Output Tokens | Total |
|----------|--------------|---------------|-------|
| Segment | ~2000 | ~500 | ~2500 |
| L1 Summary | ~2000 | ~300 | ~2300 |
| L2 Summary | ~500 | ~200 | ~700 |

**Per book (15 chapters, 5 KP/chapter):**
- Segment: 15 × 2500 = 37,500
- L1: 15 × 2300 = 34,500
- L2: 75 × 700 = 52,500
- **Total: ~125,000 tokens**

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `NATS_URL` | NATS server | `nats://localhost:4222` |
| `LLM_PROVIDER` | Primary provider | `ollama` |
| `OLLAMA_URL` | Ollama server | `http://ollama:11434` |
| `OLLAMA_MODEL` | Ollama model | `qwen2.5:14b` |
| `OPENAI_API_KEY` | OpenAI key | (production) |
| `OPENAI_MODEL` | OpenAI model | `gpt-4o-mini` |
| `LLM_TEMPERATURE` | Generation temp | `0.3` |
| `LLM_MAX_RETRIES` | Retry count | `3` |
| `LLM_TIMEOUT` | Request timeout | `120` |

---

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| litellm | 1.30+ | LLM client |
| nats-py | 2.6+ | NATS client |
| pydantic | 2.0+ | Data validation |
| jinja2 | 3.1+ | Prompt templates |
