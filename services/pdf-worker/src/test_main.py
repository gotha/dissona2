"""Tests for main module job handling."""
import asyncio
import json
from unittest.mock import AsyncMock, MagicMock, patch

import pytest


class TestHandleJob:
    """Test the handle_job function from main.py."""

    @patch("main.PdfProcessor")
    @patch("main.Settings")
    async def test_handle_job_success(self, mock_settings_cls, mock_processor_cls):
        """Successful job processing acks the message."""
        from main import handle_job

        mock_processor = MagicMock()
        mock_processor.process = AsyncMock(return_value={"document_id": "d1", "chapters": 3})
        mock_processor_cls.return_value = mock_processor

        msg = MagicMock()
        msg.data = json.dumps({
            "job_id": "j1",
            "document_id": "d1",
            "project_id": "p1",
            "file_path": "p1/original.pdf",
        }).encode()
        msg.ack = AsyncMock()

        processor = mock_processor
        await handle_job(msg, processor)

        processor.process.assert_called_once()
        msg.ack.assert_called_once()

    @patch("main.PdfProcessor")
    @patch("main.Settings")
    async def test_handle_job_failure_naks(self, mock_settings_cls, mock_processor_cls):
        """Failed job processing naks the message."""
        from main import handle_job

        mock_processor = MagicMock()
        mock_processor.process = AsyncMock(side_effect=Exception("Parse error"))
        mock_processor_cls.return_value = mock_processor

        msg = MagicMock()
        msg.data = json.dumps({
            "job_id": "j1",
            "document_id": "d1",
            "project_id": "p1",
            "file_path": "p1/original.pdf",
        }).encode()
        msg.nak = AsyncMock()

        processor = mock_processor
        await handle_job(msg, processor)

        msg.nak.assert_called_once()

    @patch("main.PdfProcessor")
    @patch("main.Settings")
    async def test_handle_job_invalid_json(self, mock_settings_cls, mock_processor_cls):
        """Invalid JSON naks the message."""
        from main import handle_job

        mock_processor = MagicMock()

        msg = MagicMock()
        msg.data = b"not valid json"
        msg.nak = AsyncMock()

        await handle_job(msg, mock_processor)

        msg.nak.assert_called_once()
