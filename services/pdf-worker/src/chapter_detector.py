import re
from dataclasses import dataclass
from typing import List, Optional

import fitz
import structlog

from config import Settings

logger = structlog.get_logger()

# Chapters larger than this will be split
MAX_CHAPTER_WORDS = 15_000
# Target size for split chunks
TARGET_CHUNK_WORDS = 10_000


@dataclass
class Chapter:
    title: Optional[str]
    text: str
    start_page: int
    end_page: int


class ChapterDetector:
    """Detects chapter boundaries in PDFs using multiple strategies."""

    def __init__(self, settings: Settings):
        self.settings = settings
        self.last_method = "unknown"

    def detect_sync(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Detect chapters using the best available method (synchronous)."""
        return self._detect(doc, full_text)

    async def detect(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Detect chapters using the best available method (async wrapper)."""
        return self._detect(doc, full_text)

    def _detect(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Core chapter detection logic."""

        # Strategy 1: Try TOC (level 1)
        chapters = self._detect_from_toc(doc, max_level=1)
        if chapters and len(chapters) > 1:
            self.last_method = "toc"
            # Check for oversized chapters and refine
            chapters = self._refine_large_chapters(doc, chapters)
            # Eliminate empty chapters by prefixing children
            chapters = self._eliminate_empty_chapters(chapters)
            return chapters

        # Strategy 2: Try heading patterns
        chapters = self._detect_from_headings(doc, full_text)
        if chapters and len(chapters) > 1:
            self.last_method = "headings"
            chapters = self._refine_large_chapters(doc, chapters)
            chapters = self._eliminate_empty_chapters(chapters)
            return chapters

        # Strategy 3: Try text patterns
        chapters = self._detect_from_patterns(full_text)
        if chapters and len(chapters) > 1:
            self.last_method = "patterns"
            chapters = self._split_oversized_chapters(chapters)
            chapters = self._eliminate_empty_chapters(chapters)
            return chapters

        # Strategy 4: Fall back to single chapter (may be split)
        self.last_method = "single"
        single = [Chapter(
            title="Full Document",
            text=full_text,
            start_page=0,
            end_page=len(doc) - 1,
        )]
        return self._split_oversized_chapters(single)

    def _refine_large_chapters(self, doc: fitz.Document, chapters: List[Chapter]) -> List[Chapter]:
        """Refine oversized chapters using TOC sub-levels, then word-count split."""
        toc = doc.get_toc()
        result = []

        for ch in chapters:
            word_count = len(ch.text.split())
            if word_count <= MAX_CHAPTER_WORDS:
                result.append(ch)
                continue

            logger.info("Chapter too large, refining",
                        title=ch.title, words=word_count)

            # Try TOC level-2 entries within this chapter's page range
            sub_chapters = self._get_toc_subchapters(
                doc, toc, ch.start_page, ch.end_page
            )

            if sub_chapters and len(sub_chapters) > 1:
                logger.info("Split via TOC sub-levels",
                            title=ch.title, sub_chapters=len(sub_chapters))
                # Recursively check sub-chapters for size
                for sc in sub_chapters:
                    if len(sc.text.split()) > MAX_CHAPTER_WORDS:
                        result.extend(self._split_by_words(sc))
                    else:
                        result.append(sc)
                self.last_method = "toc_refined"
            else:
                # No sub-levels; split by word count
                result.extend(self._split_by_words(ch))
                if self.last_method == "toc":
                    self.last_method = "toc_split"

        return result

    def _get_toc_subchapters(
        self, doc: fitz.Document, toc: list,
        start_page: int, end_page: int
    ) -> List[Chapter]:
        """Get TOC level-2+ entries within a page range."""
        entries = []
        for entry in toc:
            level, title, page = entry
            page_idx = page - 1  # TOC pages are 1-based
            if level >= 2 and start_page <= page_idx <= end_page:
                entries.append((title, page_idx))

        if len(entries) < 2:
            return []

        chapters = []
        for i, (title, page_idx) in enumerate(entries):
            ch_end = entries[i + 1][1] - 1 if i + 1 < len(entries) else end_page
            text = self._extract_pages(doc, page_idx, ch_end)
            chapters.append(Chapter(
                title=title,
                text=text,
                start_page=page_idx,
                end_page=ch_end,
            ))

        return chapters

    def _split_oversized_chapters(self, chapters: List[Chapter]) -> List[Chapter]:
        """Split any chapters exceeding MAX_CHAPTER_WORDS."""
        result = []
        for ch in chapters:
            if len(ch.text.split()) > MAX_CHAPTER_WORDS:
                result.extend(self._split_by_words(ch))
            else:
                result.append(ch)
        return result

    # Minimum word count to consider a chapter non-empty
    EMPTY_THRESHOLD = 10

    def _eliminate_empty_chapters(self, chapters: List[Chapter]) -> List[Chapter]:
        """Drop empty chapters and prefix their title onto children within their page range.

        When a chapter has effectively no content (< EMPTY_THRESHOLD words),
        it's likely a section heading whose real content lives in the chapters
        that follow. We drop the empty chapter and prepend its title to each
        subsequent chapter that falls within the empty parent's page range,
        using ' — ' as the separator.

        Example:
            "1.2 What problems..." (pages 10-15, 0 words)  → dropped
            "1.2.1 Building the software right" (pages 10-12, 758 words)
                → "1.2 What problems... — 1.2.1 Building the software right"
        """
        if not chapters:
            return chapters

        result: List[Chapter] = []
        pending_parent_title: Optional[str] = None
        pending_parent_end_page: int = -1

        for ch in chapters:
            word_count = len(ch.text.split())

            if word_count < self.EMPTY_THRESHOLD:
                # This chapter is empty — save its title for prefixing
                pending_parent_title = ch.title
                pending_parent_end_page = ch.end_page
                logger.info("Dropping empty chapter, will prefix children",
                            title=ch.title, words=word_count,
                            end_page=ch.end_page)
                continue

            if pending_parent_title and ch.start_page <= pending_parent_end_page:
                # Child is within the parent's page range — prefix it
                ch = Chapter(
                    title=f"{pending_parent_title} — {ch.title}",
                    text=ch.text,
                    start_page=ch.start_page,
                    end_page=ch.end_page,
                )
            else:
                # Outside parent's range — clear the pending prefix
                pending_parent_title = None
                pending_parent_end_page = -1

            result.append(ch)

        # If the last chapter(s) were empty, they're just dropped
        if pending_parent_title and not result:
            # Edge case: ALL chapters were empty — shouldn't happen, but return as-is
            return chapters

        return result

    def _split_by_words(self, chapter: Chapter) -> List[Chapter]:
        """Split a chapter into ~TARGET_CHUNK_WORDS chunks at paragraph boundaries."""
        text = chapter.text
        paragraphs = re.split(r"\n\s*\n", text)

        chunks: List[Chapter] = []
        current_paragraphs: List[str] = []
        current_words = 0
        part = 1

        for para in paragraphs:
            para_words = len(para.split())
            if current_words + para_words > TARGET_CHUNK_WORDS and current_paragraphs:
                # Emit current chunk
                chunk_text = "\n\n".join(current_paragraphs)
                chunks.append(Chapter(
                    title=f"{chapter.title} (Part {part})",
                    text=chunk_text,
                    start_page=chapter.start_page,
                    end_page=chapter.end_page,
                ))
                part += 1
                current_paragraphs = [para]
                current_words = para_words
            else:
                current_paragraphs.append(para)
                current_words += para_words

        # Emit remaining
        if current_paragraphs:
            chunk_text = "\n\n".join(current_paragraphs)
            # If only one part total, don't add "(Part 1)"
            title = f"{chapter.title} (Part {part})" if part > 1 else chapter.title
            chunks.append(Chapter(
                title=title,
                text=chunk_text,
                start_page=chapter.start_page,
                end_page=chapter.end_page,
            ))

        logger.info("Split large chapter by word count",
                     title=chapter.title,
                     original_words=len(text.split()),
                     parts=len(chunks))
        return chunks

    def _detect_from_toc(self, doc: fitz.Document, max_level: int = 1) -> List[Chapter]:
        """Extract chapters from PDF TOC (table of contents)."""
        toc = doc.get_toc()
        if not toc:
            return []

        chapters = []
        for i, entry in enumerate(toc):
            level, title, page = entry

            if level > max_level:
                continue

            # Find end page
            end_page = len(doc) - 1
            for j in range(i + 1, len(toc)):
                if toc[j][0] <= max_level:
                    end_page = toc[j][2] - 1
                    break

            # Extract text for this chapter
            text = self._extract_pages(doc, page - 1, end_page)

            chapters.append(Chapter(
                title=title,
                text=text,
                start_page=page - 1,
                end_page=end_page,
            ))

        return chapters

    def _detect_from_headings(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Detect chapters from heading formatting."""
        # TODO: Implement font-based detection
        return []

    def _detect_from_patterns(self, full_text: str) -> List[Chapter]:
        """Detect chapters from text patterns like 'Chapter 1' or 'PART I'."""
        patterns = [
            r"(?:^|\n)(?:Chapter|CHAPTER)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)",
            r"(?:^|\n)(?:Part|PART)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)",
            r"(?:^|\n)(\d+)\.\s+(.+?)(?=\n)",
        ]

        for pattern in patterns:
            matches = list(re.finditer(pattern, full_text))
            if len(matches) >= 2:
                return self._chapters_from_matches(matches, full_text)

        return []

    def _chapters_from_matches(self, matches, full_text: str) -> List[Chapter]:
        """Create chapters from regex matches."""
        chapters = []

        for i, match in enumerate(matches):
            title = match.group(0).strip()
            start = match.end()
            end = matches[i + 1].start() if i + 1 < len(matches) else len(full_text)

            chapters.append(Chapter(
                title=title,
                text=full_text[start:end].strip(),
                start_page=0,
                end_page=0,
            ))

        return chapters

    def _extract_pages(self, doc: fitz.Document, start: int, end: int) -> str:
        """Extract text from a range of pages."""
        text_parts = []
        for page_num in range(start, min(end + 1, len(doc))):
            text_parts.append(doc[page_num].get_text())
        return "\n\n".join(text_parts)
