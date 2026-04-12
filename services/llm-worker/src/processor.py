import json
from typing import List

import litellm
import structlog
from prometheus_client import Counter, Histogram

from config import Settings

logger = structlog.get_logger()

# Metrics
LLM_REQUESTS = Counter("llm_requests_total", "Total LLM requests", ["task_type", "status"])
LLM_DURATION = Histogram("llm_request_duration_seconds", "LLM request duration", ["task_type"])
LLM_TOKENS = Counter("llm_tokens_total", "Total LLM tokens", ["type"])


class LlmProcessor:
    def __init__(self, settings: Settings):
        self.settings = settings

        # Configure LiteLLM to use external proxy
        litellm.api_base = settings.litellm_url

    async def segment_text(self, job: dict) -> dict:
        """Split chapter text into logical segments."""
        chapter_id = job["chapter_id"]
        text = job["text"]

        prompt = f"""Split the following text into logical segments. Each segment should be a coherent unit of thought, typically 1-3 paragraphs.

Return a JSON array where each element has:
- "text": the segment text
- "start_char": character offset where segment starts
- "end_char": character offset where segment ends

Text:
{text[:8000]}  # Limit to avoid context length issues

Return ONLY valid JSON, no other text."""

        with LLM_DURATION.labels(task_type="segment").time():
            response = await self._call_llm(prompt)

        try:
            segments = json.loads(response)
            LLM_REQUESTS.labels(task_type="segment", status="success").inc()
            return {
                "chapter_id": chapter_id,
                "segments": segments,
            }
        except json.JSONDecodeError:
            LLM_REQUESTS.labels(task_type="segment", status="error").inc()
            # Fallback: split by paragraphs
            return {
                "chapter_id": chapter_id,
                "segments": self._fallback_segment(text),
            }

    async def summarize(self, job: dict) -> dict:
        """Generate a summary for text."""
        text = job["text"]
        summary_type = job.get("summary_type", "chapter")

        if summary_type == "chapter":
            prompt = f"""Summarize the following chapter in 2-3 concise paragraphs. 
Focus on the main ideas and key takeaways.

Text:
{text[:8000]}

Summary:"""
        else:
            prompt = f"""Summarize the following key point in 1-2 sentences.
Be concise and capture the essential insight.

Text:
{text[:4000]}

Summary:"""

        with LLM_DURATION.labels(task_type="summarize").time():
            summary = await self._call_llm(prompt)

        LLM_REQUESTS.labels(task_type="summarize", status="success").inc()

        return {
            "chapter_id": job.get("chapter_id"),
            "key_point_id": job.get("key_point_id"),
            "summary": summary.strip(),
        }

    async def extract_keypoints(self, job: dict) -> dict:
        """Extract key points from chapter segments."""
        chapter_id = job["chapter_id"]
        segments = job["segments"]

        # Format segments for prompt
        segments_text = "\n\n".join(
            f"[Segment {i + 1}]: {s['text'][:500]}" for i, s in enumerate(segments[:20])
        )

        prompt = f"""Identify the key points or main topics in these text segments.
Group consecutive segments that discuss the same topic.

Return a JSON array where each element has:
- "title": short title for the key point (max 50 chars)
- "segment_start": first segment number (1-indexed)
- "segment_end": last segment number (1-indexed)

Segments:
{segments_text}

Return ONLY valid JSON, no other text."""

        with LLM_DURATION.labels(task_type="keypoints").time():
            response = await self._call_llm(prompt)

        try:
            keypoints = json.loads(response)
            LLM_REQUESTS.labels(task_type="keypoints", status="success").inc()
            return {
                "chapter_id": chapter_id,
                "key_points": keypoints,
            }
        except json.JSONDecodeError:
            LLM_REQUESTS.labels(task_type="keypoints", status="error").inc()
            # Fallback: one key point per 3 segments
            return {
                "chapter_id": chapter_id,
                "key_points": self._fallback_keypoints(segments),
            }

    async def _call_llm(self, prompt: str) -> str:
        """Call LLM via LiteLLM."""
        response = await litellm.acompletion(
            model=self.settings.llm_model,
            messages=[{"role": "user", "content": prompt}],
            temperature=self.settings.llm_temperature,
            max_tokens=self.settings.llm_max_tokens,
        )

        # Track tokens
        if hasattr(response, "usage"):
            LLM_TOKENS.labels(type="prompt").inc(response.usage.prompt_tokens)
            LLM_TOKENS.labels(type="completion").inc(response.usage.completion_tokens)

        return response.choices[0].message.content

    def _fallback_segment(self, text: str) -> List[dict]:
        """Fallback segmentation by paragraphs."""
        paragraphs = text.split("\n\n")
        segments = []
        offset = 0

        for para in paragraphs:
            if para.strip():
                segments.append(
                    {
                        "text": para.strip(),
                        "start_char": offset,
                        "end_char": offset + len(para),
                    }
                )
            offset += len(para) + 2

        return segments

    def _fallback_keypoints(self, segments: List[dict]) -> List[dict]:
        """Fallback: group every 3 segments into a key point."""
        keypoints = []
        for i in range(0, len(segments), 3):
            keypoints.append(
                {
                    "title": f"Section {len(keypoints) + 1}",
                    "segment_start": i + 1,
                    "segment_end": min(i + 3, len(segments)),
                }
            )
        return keypoints
