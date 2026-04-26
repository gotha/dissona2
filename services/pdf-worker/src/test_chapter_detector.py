"""Tests for chapter detection logic."""
import fitz
import pytest

from chapter_detector import Chapter, ChapterDetector, MAX_CHAPTER_WORDS, TARGET_CHUNK_WORDS
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
        assert len(chapters) == 3
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
        detector = ChapterDetector(settings)
        text = "\nChapter 1: Introduction\nContent one.\n\nChapter 2: Background\nContent two.\n\nChapter 3: Methods\nContent three.\n"
        chapters = detector._detect_from_patterns(text)
        assert len(chapters) == 3
        assert "Introduction" in chapters[0].title

    def test_detect_from_patterns_part_style(self, settings):
        detector = ChapterDetector(settings)
        text = "\nPart 1: The Beginning\nText.\n\nPart 2: The Middle\nText.\n\nPart 3: The End\nText.\n"
        chapters = detector._detect_from_patterns(text)
        assert len(chapters) == 3

    def test_detect_from_patterns_numbered_style(self, settings):
        detector = ChapterDetector(settings)
        text = "\n1. First Section\nContent.\n\n2. Second Section\nContent.\n\n3. Third Section\nContent.\n"
        chapters = detector._detect_from_patterns(text)
        assert len(chapters) == 3

    def test_detect_from_patterns_no_match(self, settings):
        detector = ChapterDetector(settings)
        chapters = detector._detect_from_patterns("Just plain text.")
        assert chapters == []

    def test_detect_from_headings_returns_empty(self, settings, sample_pdf_doc):
        detector = ChapterDetector(settings)
        chapters = detector._detect_from_headings(sample_pdf_doc, "text")
        assert chapters == []

    def test_extract_pages(self, settings, sample_pdf_doc):
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 0)
        assert len(text) > 0

    def test_extract_pages_range(self, settings, sample_pdf_doc):
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 2)
        assert len(text) > 0

    def test_extract_pages_clamps_to_doc_length(self, settings, sample_pdf_doc):
        detector = ChapterDetector(settings)
        text = detector._extract_pages(sample_pdf_doc, 0, 100)
        assert len(text) > 0

    def test_async_detect(self, settings, single_page_pdf):
        import asyncio
        detector = ChapterDetector(settings)
        doc = fitz.open(stream=single_page_pdf, filetype="pdf")
        text = doc[0].get_text()
        chapters = asyncio.run(detector.detect(doc, text))
        doc.close()
        assert len(chapters) == 1
        assert detector.last_method == "single"

    def test_chapters_from_matches_text_boundaries(self, settings):
        import re
        detector = ChapterDetector(settings)
        text = "Chapter 1: Intro\nFirst content.\nChapter 2: Body\nSecond content."
        matches = list(re.finditer(r"(?:^|\n)(?:Chapter|CHAPTER)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)", text))
        chapters = detector._chapters_from_matches(matches, text)
        assert len(chapters) == 2
        assert "First content" in chapters[0].text
        assert "Second content" in chapters[1].text


class TestSplitByWords:
    """Tests for word-count based chapter splitting."""

    def test_small_chapter_not_split(self, settings):
        detector = ChapterDetector(settings)
        ch = Chapter(title="Small", text="Just a few words.", start_page=0, end_page=0)
        result = detector._split_by_words(ch)
        assert len(result) == 1
        assert result[0].title == "Small"

    def test_large_chapter_split_into_parts(self, settings):
        detector = ChapterDetector(settings)
        # Create text with ~25K words (should split into ~3 parts)
        paragraphs = [f"Paragraph {i}. " + ("word " * 500) for i in range(50)]
        text = "\n\n".join(paragraphs)
        ch = Chapter(title="Huge Chapter", text=text, start_page=0, end_page=10)
        result = detector._split_by_words(ch)

        assert len(result) > 1
        assert result[0].title == "Huge Chapter (Part 1)"
        assert result[1].title == "Huge Chapter (Part 2)"
        # Each part should be <= TARGET_CHUNK_WORDS (with some tolerance for paragraph boundaries)
        for part in result[:-1]:
            assert len(part.text.split()) <= TARGET_CHUNK_WORDS + 600

    def test_split_preserves_page_info(self, settings):
        detector = ChapterDetector(settings)
        paragraphs = [("word " * 600) for _ in range(30)]
        text = "\n\n".join(paragraphs)
        ch = Chapter(title="Big", text=text, start_page=5, end_page=20)
        result = detector._split_by_words(ch)
        for part in result:
            assert part.start_page == 5
            assert part.end_page == 20

    def test_split_single_huge_paragraph(self, settings):
        """A single paragraph larger than target stays as one chunk."""
        detector = ChapterDetector(settings)
        text = "word " * 20000
        ch = Chapter(title="One Block", text=text, start_page=0, end_page=0)
        result = detector._split_by_words(ch)
        # Can't split a single paragraph, so it stays as one
        assert len(result) == 1
        assert result[0].title == "One Block"


class TestRefineAndSplitOversized:
    """Tests for _refine_large_chapters and _split_oversized_chapters."""

    def test_small_chapters_unchanged(self, settings):
        detector = ChapterDetector(settings)
        chapters = [
            Chapter(title="Ch 1", text="short text", start_page=0, end_page=1),
            Chapter(title="Ch 2", text="also short", start_page=2, end_page=3),
        ]
        result = detector._split_oversized_chapters(chapters)
        assert len(result) == 2
        assert result[0].title == "Ch 1"

    def test_oversized_chapter_is_split(self, settings):
        detector = ChapterDetector(settings)
        big_text = "\n\n".join(["word " * 600 for _ in range(40)])
        chapters = [
            Chapter(title="Small", text="tiny", start_page=0, end_page=0),
            Chapter(title="Huge", text=big_text, start_page=1, end_page=50),
        ]
        result = detector._split_oversized_chapters(chapters)
        assert len(result) > 2
        assert result[0].title == "Small"
        assert "Huge (Part 1)" in result[1].title

    def test_refine_uses_toc_sublevels(self, settings):
        """When a chapter is too large, try TOC level-2 entries."""
        detector = ChapterDetector(settings)

        # Build a PDF with TOC containing level-1 and level-2 entries
        doc = fitz.open()
        for i in range(10):
            p = doc.new_page()
            p.insert_text((72, 72), f"Content for page {i+1}. " + ("word " * 200))
        doc.set_toc([
            [1, "Part One", 1],
            [2, "Section A", 1],
            [2, "Section B", 4],
            [2, "Section C", 7],
            [1, "Part Two", 10],
        ])

        text = "\n\n".join(page.get_text() for page in doc)
        toc = doc.get_toc()

        # Create a large chapter spanning pages 0-8 (Part One)
        big_text = "\n\n".join(doc[i].get_text() for i in range(9))
        ch = Chapter(title="Part One", text=big_text, start_page=0, end_page=8)

        sub_chapters = detector._get_toc_subchapters(doc, toc, 0, 8)
        doc.close()

        assert len(sub_chapters) == 3
        assert sub_chapters[0].title == "Section A"
        assert sub_chapters[1].title == "Section B"
        assert sub_chapters[2].title == "Section C"

    def test_get_toc_subchapters_no_level2(self, settings):
        """No level-2 entries returns empty list."""
        detector = ChapterDetector(settings)
        doc = fitz.open()
        doc.new_page()
        doc.set_toc([[1, "Only Level 1", 1]])
        toc = doc.get_toc()
        result = detector._get_toc_subchapters(doc, toc, 0, 0)
        doc.close()
        assert result == []

    def test_get_toc_subchapters_single_level2(self, settings):
        """Only one level-2 entry returns empty (need >=2)."""
        detector = ChapterDetector(settings)
        doc = fitz.open()
        doc.new_page()
        doc.set_toc([[1, "Main", 1], [2, "Sub A", 1]])
        toc = doc.get_toc()
        result = detector._get_toc_subchapters(doc, toc, 0, 0)
        doc.close()
        assert result == []
