"""Tests for PDF processor."""
import io
from unittest.mock import MagicMock, patch, PropertyMock

import fitz
import pytest

from chapter_detector import Chapter
from config import Settings
from processor import PdfProcessor


class TestPdfProcessor:
    """Tests for PdfProcessor."""

    def test_init(self, settings):
        with patch("processor.boto3") as mock_boto:
            processor = PdfProcessor(settings)
            mock_boto.client.assert_called_once_with(
                "s3",
                endpoint_url=settings.s3_endpoint,
                aws_access_key_id=settings.s3_access_key,
                aws_secret_access_key=settings.s3_secret_key,
            )

    def test_download_pdf(self, settings, sample_pdf_bytes):
        with patch("processor.boto3") as mock_boto:
            mock_s3 = MagicMock()
            mock_boto.client.return_value = mock_s3
            mock_body = MagicMock()
            mock_body.read.return_value = sample_pdf_bytes
            mock_s3.get_object.return_value = {"Body": mock_body}

            processor = PdfProcessor(settings)
            data = processor._download_pdf("test-key")

            mock_s3.get_object.assert_called_once_with(
                Bucket=settings.s3_bucket_uploads,
                Key="test-key",
            )
            assert data == sample_pdf_bytes

    def test_extract_text(self, settings, sample_pdf_bytes):
        with patch("processor.boto3"):
            processor = PdfProcessor(settings)
            doc = fitz.open(stream=sample_pdf_bytes, filetype="pdf")
            text, page_count = processor._extract_text(doc)
            doc.close()

            assert page_count == 3
            assert len(text) > 0
            assert "Introduction" in text or "Chapter" in text

    def test_extract_text_single_page(self, settings, single_page_pdf):
        with patch("processor.boto3"):
            processor = PdfProcessor(settings)
            doc = fitz.open(stream=single_page_pdf, filetype="pdf")
            text, page_count = processor._extract_text(doc)
            doc.close()

            assert page_count == 1
            assert "single page" in text

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_update_document_status(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        processor = PdfProcessor(settings)
        processor._update_document_status("doc-123", "processing", "parsing")

        cursor.execute.assert_called_once()
        call_args = cursor.execute.call_args
        assert "UPDATE documents" in call_args[0][0]
        assert call_args[0][1] == ("processing", "parsing", "doc-123")
        conn.commit.assert_called_once()
        conn.close.assert_called_once()

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_save_results(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        processor = PdfProcessor(settings)
        chapters = [
            Chapter(title="Ch 1", text="Text one", start_page=0, end_page=0),
            Chapter(title="Ch 2", text="Text two words", start_page=1, end_page=1),
        ]
        processor._save_results(
            document_id="doc-1",
            project_id="proj-1",
            page_count=2,
            detection_method="toc",
            chapters=chapters,
            pdf_title="Book Title",
            pdf_author="Author Name",
        )

        # Should have: 1 doc update + 2 chapter inserts + 1 project update = 4 calls
        assert cursor.execute.call_count == 4
        conn.commit.assert_called_once()
        conn.close.assert_called_once()

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_save_results_chapter_word_count(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        processor = PdfProcessor(settings)
        chapters = [
            Chapter(title="Ch", text="one two three four five", start_page=0, end_page=0),
        ]
        processor._save_results(
            document_id="doc-1", project_id="proj-1", page_count=1,
            detection_method="single", chapters=chapters, pdf_title="", pdf_author="",
        )

        # Check the chapter INSERT call has correct word count
        chapter_insert = cursor.execute.call_args_list[1]
        assert chapter_insert[0][1][6] == 5  # word_count

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_mark_failed(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        processor = PdfProcessor(settings)
        processor._mark_failed("doc-1", "proj-1", "Something broke")

        assert cursor.execute.call_count == 2
        # Document update
        doc_call = cursor.execute.call_args_list[0]
        assert "failed" in doc_call[0][0]
        assert doc_call[0][1] == ("Something broke", "doc-1")
        # Project update
        proj_call = cursor.execute.call_args_list[1]
        assert "failed" in proj_call[0][0]
        conn.commit.assert_called_once()

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_mark_failed_handles_db_error(self, mock_boto, mock_psycopg2, settings):
        mock_psycopg2.connect.side_effect = Exception("DB down")

        processor = PdfProcessor(settings)
        # Should not raise
        processor._mark_failed("doc-1", "proj-1", "error")
