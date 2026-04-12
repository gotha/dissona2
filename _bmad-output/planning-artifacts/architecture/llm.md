# LLM Infrastructure Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **LLM Backend** | Ollama (external) |
| **Proxy** | LiteLLM |
| **Ollama Host** | 10.100.0.100 |
| **LiteLLM Port** | 14000 |

Ollama runs on a separate GPU server, accessed via LiteLLM proxy.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     DOCKER COMPOSE                               │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  PDF Worker  │  │  LLM Worker  │  │  TTS Worker  │          │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┘          │
│         │                 │                                      │
└─────────┼─────────────────┼──────────────────────────────────────┘
          │                 │
          │    HTTP         │
          ▼                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                   10.100.0.100                                   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    LiteLLM (port 14000)                   │   │
│  │                                                           │   │
│  │  • OpenAI-compatible API                                  │   │
│  │  • Model routing                                          │   │
│  │  • Request logging                                        │   │
│  └────────────────────────────┬─────────────────────────────┘   │
│                               │                                  │
│                               ▼                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    Ollama (port 11434)                    │   │
│  │                                                           │   │
│  │  • qwen2.5:14b (LLM)                                      │   │
│  │  • nomic-embed-text (embeddings)                          │   │
│  │  • GPU: NVIDIA RTX                                        │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Models

| Model | Size | Purpose |
|-------|------|---------|
| `qwen2.5:14b` | ~10GB | Summarization, segmentation |
| `nomic-embed-text` | ~275MB | Semantic chunking (fallback) |

---

## LiteLLM Configuration

Workers call LiteLLM at `http://10.100.0.100:14000`:

```python
import litellm

litellm.api_base = "http://10.100.0.100:14000"

response = await litellm.acompletion(
    model="qwen2.5:14b",
    messages=messages,
    temperature=0.3,
)
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `LITELLM_URL` | LiteLLM proxy endpoint | `http://10.100.0.100:14000` |
| `LLM_MODEL` | Model name | `qwen2.5:14b` |
| `LLM_TEMPERATURE` | Generation temperature | `0.3` |
| `LLM_MAX_TOKENS` | Max output tokens | `4096` |

---

## Production Migration

Switch to OpenAI by changing model name:

```python
# Dev (via LiteLLM → Ollama)
model = "qwen2.5:14b"

# Production (via LiteLLM → OpenAI)
model = "gpt-4o-mini"
```

LiteLLM handles routing based on model name.
