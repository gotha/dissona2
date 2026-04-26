"""Shared test fixtures for pdf-worker tests."""
import pytest
from unittest.mock import MagicMock, patch

import fitz

from config import Settings


@pytest.fixture
def settings():
    """Test settings with defaults."""
    return Settings(
        nats_url="nats://localhost:4222",
        database_url="postgresql://test:test@localhost/test",
        s3_endpoint="http://localhost:9000",
        s3_access_key="test",
        s3_secret_key="test",
        s3_bucket_uploads="test-bucket",
        ocr_enabled=False,
        metrics_port=0,
    )


@pytest.fixture
def sample_pdf_bytes():
    """Create a minimal valid PDF in memory."""
    doc = fitz.open()
    page1 = doc.new_page()
    page1.insert_text((72, 72), "Chapter 1: Introduction\n\nThis is the first chapter with some text content.")
    page2 = doc.new_page()
    page2.insert_text((72, 72), "Chapter 2: Methods\n\nThis is the second chapter about methods.")
    page3 = doc.new_page()
    page3.insert_text((72, 72), "Chapter 3: Conclusion\n\nThis is the final chapter.")
    data = doc.tobytes()
    doc.close()
    return data


@pytest.fixture
def sample_pdf_doc(sample_pdf_bytes):
    """Open a sample PDF as fitz.Document."""
    doc = fitz.open(stream=sample_pdf_bytes, filetype="pdf")
    yield doc
    doc.close()


@pytest.fixture
def sample_pdf_with_toc():
    """Create a PDF with a TOC."""
    doc = fitz.open()
    page1 = doc.new_page()
    page1.insert_text((72, 72), "Foreword\n\nSome introductory text here.")
    page2 = doc.new_page()
    page2.insert_text((72, 72), "Main Content\n\nThe body of the document.")
    page3 = doc.new_page()
    page3.insert_text((72, 72), "Appendix\n\nAdditional materials.")
    doc.set_toc([
        [1, "Foreword", 1],
        [1, "Main Content", 2],
        [2, "Subsection A", 2],
        [1, "Appendix", 3],
    ])
    data = doc.tobytes()
    doc.close()
    return data


@pytest.fixture
def sample_pdf_with_metadata():
    """Create a PDF with metadata."""
    doc = fitz.open()
    page = doc.new_page()
    page.insert_text((72, 72), "Hello World")
    doc.set_metadata({
        "title": "Test Book Title",
        "author": "Test Author",
        "subject": "Testing",
    })
    data = doc.tobytes()
    doc.close()
    return data


@pytest.fixture
def single_page_pdf():
    """Create a single page PDF with no chapter markers."""
    doc = fitz.open()
    page = doc.new_page()
    page.insert_text((72, 72), "Just a single page of text with no chapter structure at all.")
    data = doc.tobytes()
    doc.close()
    return data


@pytest.fixture
def mock_db_conn():
    """Mock database connection."""
    conn = MagicMock()
    cursor = MagicMock()
    conn.cursor.return_value.__enter__ = MagicMock(return_value=cursor)
    conn.cursor.return_value.__exit__ = MagicMock(return_value=False)
    return conn, cursor
