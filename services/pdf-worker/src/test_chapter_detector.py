"""Tests for chapter detection logic."""
import fitz
import pytest

from chapter_detector import Chapter, ChapterDetector
from config import Settings


class TestChapterDetector:
    """Tests for ChapterDetector."""

    def test_init(self, settings):
        detector = ChapterDetector(settings)
        assert detector.last_method == "unknown"

    def test_single_chapter_fallback(self, settings, single_page_pdf):
        """When no chapters detected, fall back to single chapter."""
        detector = ChapterDetector(settings)
        doc = fitz.open(stream=single_page_pdf, filetype="pdf")
        text = doc[0].get_text()
        chapters = detector.detect_sync(doc, text)
        doc.close()

        assert len(chapters) == 1
        assert chapters[0].title == "Full Document"
        assert detector.last_method == "single"
        assert chapters[0].start_page == 0
        assert chapters[0].end_page == 0

    def test_detect_from_toc(self, settings, sample_pdf_with_toc):
        """Detect chapters from PDF table of contents."""
        detector = ChapterDetector(settings)
        doc = fitz.open(stream=sample_pdf_with_toc, filetype="pdf")
        text = "\n\n".join(page.get_text() for page in doc)
        chapters = detector.detect_sync(doc, text)
        doc.close()

        assert detector.last_method == "toc"
        assert len(chapters) == 3  # Foreword, Main Content, Appendix (level 1 only)
        assert chapters[0].title == "Foreword"
        assert chapters[1].title == "Main Content"
        assert chapters[2].title == "Appendix"

    def test_toc_skips_sublevels(self, settings, sample_pdf_with_toc):
        """TOC detection only uses level-1 entries."""
        detector = ChapterDetector(settings)
        doc = fitz.open(stream=sample_pdf_with_toc, filetype="pdf")
        text = "\n\n".join(page.get_text() for page in doc)
        chapters = detector.detect_sync(doc, text)
        doc.close()

        titles = [ch.title for ch in chapters]
        assert "Subsection A" not in titles

    def test_detect_from_patterns_chapter_style(self, settings):
        """Detect chapters from 'Chapter N: Title' patterns."""
        detector = ChapterDetector(settings)
        doc = fitz.open()
        doc.new_page()
        doc.close()

        text = """
Chapter 1: Introduction
This is the introduction text with enough content.

Chapter 2: Background
This is the background section with details.

Chapter 3: Methods
This describes the methodology used.
"""
        # Use a dummy doc for fallback
        doc = fitz.open()
        doc.new_page()
        chapters = detector._detect_from_patterns(text)
        doc.close()

        assert len(chapters) == 3
        assert "Introduction" in chapters[0].title
        assert "Background" in chapters[1].title
        assert "Methods" in chapters[2].title

    def test_detect_from_patterns_part_style(self, settings):
        """Detect chapters from 'Part N: Title' patterns."""
        detector = ChapterDetector(settings)

        text = """
Part 1: The Beginning
Some text for part one.

Part 2: The Middle
Some text for part two.

Part 3: The End
Some text for part three.
"""
        chapters = detector._detect_from_patterns(text)

        assert len(chapters) == 3
        assert "Beginning" in chapters[0].title
        assert "Middle" in chapters[1].title

    def test_detect_from_patterns_numbered_style(self, settings):
        """Detect chapters from '1. Title' patterns."""
        detector = ChapterDetector(settings)

        text = """
1. First Section
Content of the first section here.

2. Second Section
Content of the second section here.

3. Third Section
Content of the third section here.
"""
        chapters = detector._detect_from_patterns(text)

        assert len(chapters) == 3

    def test_detect_from_patterns_no_match(self, settings):
        """No patterns found returns empty list."""
        detector = ChapterDetector(settings)
        chapters = detector._detect_from_patterns("Just plain text without any chapter markers.")
        assert chapters == []

    def test_detect_from_headings_returns_empty(self, settings, sample_pdf_doc):
        """Heading detection is not yet implemented."""
        detector = ChapterDetector(settings)
        text = sample_pdf_doc[0].get_text()
        chapters = detector._detect_from_headings(sample_pdf_doc, text)
        assert chapters == []

    def test_extract_pages(self, settings, sample_pdf_doc):
        """Extract text from page range."""
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 0)
        assert len(text) > 0

    def test_extract_pages_range(self, settings, sample_pdf_doc):
        """Extract text from multi-page range."""
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 2)
        assert len(text) > 0

    def test_extract_pages_clamps_to_doc_length(self, settings, sample_pdf_doc):
        """Out-of-bounds end page is clamped."""
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 100)
        assert len(text) > 0

    def test_async_detect(self, settings, single_page_pdf):
        """Async detect wrapper works."""
        import asyncio
        detector = ChapterDetector(settings)
        doc = fitz.open(stream=single_page_pdf, filetype="pdf")
        text = doc[0].get_text()
        chapters = asyncio.run(detector.detect(doc, text))
        doc.close()

        assert len(chapters) == 1
        assert detector.last_method == "single"

    def test_chapters_from_matches_text_boundaries(self, settings):
        """Verify chapter text boundaries from regex matches."""
        import re
        detector = ChapterDetector(settings)
        text = "Chapter 1: Intro\nFirst content.\nChapter 2: Body\nSecond content."
        matches = list(re.finditer(r"(?:^|\n)(?:Chapter|CHAPTER)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)", text))
        chapters = detector._chapters_from_matches(matches, text)
        assert len(chapters) == 2
        assert "First content" in chapters[0].text
        assert "Second content" in chapters[1].text
