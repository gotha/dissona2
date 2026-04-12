# PDF Worker Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Python 3.11 |
| **GPU Required** | No (CPU only) |
| **Queue** | NATS JetStream |
| **Database** | None (single writer pattern) |

---

## Responsibilities

### Primary Functions

1. **PDF Download** — Fetch PDF from object storage
2. **Text Extraction** — Extract text from native PDFs (PyMuPDF)
3. **OCR** — Extract text from scanned PDFs (Tesseract)
4. **Chapter Detection** — Identify chapter boundaries
5. **Quality Assessment** — Validate extraction quality
6. **Length Normalization** — Split/merge chapters to 30-45 min targets

### What This Service Does NOT Do

- ❌ Write to database (API Service does this)
- ❌ Generate summaries (LLM Worker does this)
- ❌ Run on GPU (uses Ollama for embeddings via HTTP)

---

## Queue Interface

### Input: `pdf.parse`

```json
{
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "file_path": "uploads/book-uuid/original.pdf"
}
```

### Output: `pdf.completed`

**Success:**

```json
{
  "status": "success",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "title": "The Manager's Path",
  "author": "Camille Fournier",
  "page_count": 284,
  "detection_method": "toc",
  "chapters": [
    {
      "number": 1,
      "title": "Management 101",
      "text": "Full chapter text...",
      "word_count": 4500,
      "estimated_duration_min": 30,
      "is_split": false
    }
  ]
}
```

**Failure:**

```json
{
  "status": "failed",
  "job_id": "job-uuid",
  "book_id": "book-uuid",
  "error": "PDF is password protected",
  "error_type": "ProtectedPDF"
}
```

---

## Processing Pipeline

```
1. Download PDF from S3
         │
         ▼
2. Detect PDF Type
   ├── Native (text-based)
   ├── Scanned (image-based)
   ├── Mixed
   ├── Protected → Error
   └── Corrupted → Error
         │
         ▼
3. Extract Text
   ├── Native → PyMuPDF
   └── Scanned → Tesseract OCR (parallel)
         │
         ▼
4. Detect Chapters (priority order)
   ├── TOC → Use table of contents
   ├── Headings → Use font-based detection
   ├── Patterns → Use "Chapter X" regex
   └── Semantic → Use embeddings (Ollama)
         │
         ▼
5. Assess Quality
   ├── Chapter count OK? (5-30)
   ├── Lengths reasonable? (±50% variance)
   ├── Coverage OK? (>90%)
   └── Low quality? → Try next method
         │
         ▼
6. Normalize Lengths
   ├── > 45 min → Split at semantic boundary
   ├── < 5 min → Merge with adjacent
   └── 5-45 min → Keep as-is
         │
         ▼
7. Publish pdf.completed
```

---

## Chapter Detection Methods

### 1. Table of Contents (Highest Priority)

```python
doc = fitz.open(pdf_path)
toc = doc.get_toc()  # [(level, title, page), ...]

# Filter for top-level entries
chapters = [entry for entry in toc if entry[0] == 1]
```

### 2. Heading Detection (Font Analysis)

```python
for page in doc:
    blocks = page.get_text("dict")["blocks"]
    for block in blocks:
        for span in block["spans"]:
            if span["size"] > 16:  # Large font = heading
                headings.append(span["text"])
```

### 3. Pattern Matching (Regex)

```python
patterns = [
    r'^Chapter\s+(\d+)',
    r'^CHAPTER\s+(\d+)',
    r'^Part\s+(\d+)',
    r'^\d+\.\s+\w+',  # "1. Introduction"
]
```

### 4. Semantic Detection (Embeddings Fallback)

```python
# Split into paragraphs
paragraphs = text.split('\n\n')

# Get embeddings from Ollama
embeddings = await ollama_embed(paragraphs)

# Find similarity drops (topic changes)
for i in range(len(embeddings) - 1):
    similarity = cosine_similarity(embeddings[i], embeddings[i+1])
    if similarity < threshold:
        boundaries.append(i)
```

---

## Quality Assessment

```python
def assess_quality(chapters, total_text_length):
    score = 1.0
    issues = []
    
    # Check chapter count
    if len(chapters) < 3:
        score -= 0.3
        issues.append("Too few chapters")
    
    # Check length variance
    lengths = [len(ch["text"]) for ch in chapters]
    variance = max(lengths) / min(lengths)
    if variance > 5:
        score -= 0.2
        issues.append("High length variance")
    
    # Check coverage
    coverage = sum(lengths) / total_text_length
    if coverage < 0.8:
        score -= 0.2
        issues.append("Low text coverage")
    
    return {
        "score": max(0, score),
        "confidence": "high" if score > 0.8 else "medium" if score > 0.5 else "low",
        "issues": issues
    }
```

---

## OCR Configuration

| Setting | Value | Notes |
|---------|-------|-------|
| Engine | Tesseract 5 | CPU-based |
| DPI | 300 | Balance quality/speed |
| Language | eng | Add more as needed |
| Workers | 4 | Parallel page processing |
| OEM | 3 | Default LSTM engine |
| PSM | 1 | Auto page segmentation |

---

## Chapter Length Targets

| Metric | Target | Min | Max |
|--------|--------|-----|-----|
| Duration | 30 min | 5 min | 45 min |
| Words | 4,500 | 750 | 6,750 |

---

## Error Types

| Error | Description | User Message |
|-------|-------------|--------------|
| `CorruptedPDF` | Can't open file | "File appears corrupted" |
| `ProtectedPDF` | Password protected | "PDF is password protected" |
| `NoTextExtracted` | OCR failed | "Couldn't extract text" |
| `NoChaptersFound` | Detection failed | "Couldn't detect chapters" |
| `ExtractionFailed` | General failure | "Processing failed" |

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `NATS_URL` | NATS server | `nats://localhost:4222` |
| `S3_ENDPOINT` | Object storage | required |
| `OLLAMA_URL` | Ollama server | `http://ollama:11434` |
| `EMBEDDING_MODEL` | Ollama model | `nomic-embed-text` |
| `OCR_WORKERS` | Parallel OCR | `4` |
| `OCR_DPI` | Scan resolution | `300` |

---

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| PyMuPDF | 1.23+ | PDF parsing |
| pytesseract | 0.3+ | OCR bindings |
| pdf2image | 1.16+ | PDF to images |
| Pillow | 10+ | Image processing |
| httpx | 0.27+ | Ollama HTTP client |
| nats-py | 2.6+ | NATS client |
