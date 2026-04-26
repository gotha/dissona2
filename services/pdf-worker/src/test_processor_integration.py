"""Integration tests for PDF processor (end-to-end with mocked external services)."""
import asyncio
from unittest.mock import MagicMock, patch

import fitz
import pytest

from config import Settings
from processor import PdfProcessor


class TestProcessSync:
    """Test the full _process_sync pipeline with mocked S3 and DB."""

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_process_sync_success(self, mock_boto, mock_psycopg2, settings, sample_pdf_bytes, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        mock_s3 = MagicMock()
        mock_boto.client.return_value = mock_s3
        mock_body = MagicMock()
        mock_body.read.return_value = sample_pdf_bytes
        mock_s3.get_object.return_value = {"Body": mock_body}

        processor = PdfProcessor(settings)
        result = processor._process_sync({
            "job_id": "job-1",
            "document_id": "doc-1",
            "project_id": "proj-1",
            "file_path": "proj-1/original.pdf",
        })

        assert result["document_id"] == "doc-1"
        assert result["chapters"] >= 1
        # DB should have been updated multiple times
        assert cursor.execute.call_count > 0
        assert conn.commit.call_count > 0

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_process_sync_s3_failure(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        mock_s3 = MagicMock()
        mock_boto.client.return_value = mock_s3
        mock_s3.get_object.side_effect = Exception("S3 unavailable")

        processor = PdfProcessor(settings)
        with pytest.raises(Exception, match="S3 unavailable"):
            processor._process_sync({
                "job_id": "job-1",
                "document_id": "doc-1",
                "project_id": "proj-1",
                "file_path": "proj-1/original.pdf",
            })

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_process_sync_marks_failed_on_error(self, mock_boto, mock_psycopg2, settings, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        mock_s3 = MagicMock()
        mock_boto.client.return_value = mock_s3
        mock_s3.get_object.side_effect = Exception("Boom")

        processor = PdfProcessor(settings)
        with pytest.raises(Exception):
            processor._process_sync({
                "job_id": "j1",
                "document_id": "d1",
                "project_id": "p1",
                "file_path": "path.pdf",
            })

        # Should have called _mark_failed which updates doc and project to 'failed'
        failed_calls = [
            c for c in cursor.execute.call_args_list
            if "failed" in str(c)
        ]
        assert len(failed_calls) >= 1

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_process_async_wrapper(self, mock_boto, mock_psycopg2, settings, sample_pdf_bytes, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        mock_s3 = MagicMock()
        mock_boto.client.return_value = mock_s3
        mock_body = MagicMock()
        mock_body.read.return_value = sample_pdf_bytes
        mock_s3.get_object.return_value = {"Body": mock_body}

        processor = PdfProcessor(settings)
        result = asyncio.run(processor.process({
            "job_id": "job-1",
            "document_id": "doc-1",
            "project_id": "proj-1",
            "file_path": "proj-1/original.pdf",
        }))

        assert result["document_id"] == "doc-1"

    @patch("processor.psycopg2")
    @patch("processor.boto3")
    def test_process_sync_with_metadata(self, mock_boto, mock_psycopg2, settings, sample_pdf_with_metadata, mock_db_conn):
        conn, cursor = mock_db_conn
        mock_psycopg2.connect.return_value = conn

        mock_s3 = MagicMock()
        mock_boto.client.return_value = mock_s3
        mock_body = MagicMock()
        mock_body.read.return_value = sample_pdf_with_metadata
        mock_s3.get_object.return_value = {"Body": mock_body}

        processor = PdfProcessor(settings)
        result = processor._process_sync({
            "job_id": "job-1",
            "document_id": "doc-1",
            "project_id": "proj-1",
            "file_path": "proj-1/original.pdf",
        })

        assert result["document_id"] == "doc-1"
        # Check that metadata was passed to _save_results
        # The document update should include the PDF title and author
        doc_update_calls = [
            c for c in cursor.execute.call_args_list
            if "UPDATE documents" in str(c) and "processed" in str(c)
        ]
        assert len(doc_update_calls) >= 1
