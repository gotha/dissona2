"""Tests for config module."""
import os
import pytest
from config import Settings


class TestSettings:
    def test_explicit_values(self):
        """Settings can be constructed with explicit values."""
        s = Settings(
            nats_url="nats://custom:4222",
            database_url="postgresql://user:pass@db/mydb",
            s3_endpoint="http://s3:9000",
            s3_access_key="mykey",
            s3_secret_key="mysecret",
            s3_bucket_uploads="my-bucket",
            ocr_enabled=True,
            metrics_port=9999,
        )
        assert s.nats_url == "nats://custom:4222"
        assert s.database_url == "postgresql://user:pass@db/mydb"
        assert s.s3_endpoint == "http://s3:9000"
        assert s.s3_access_key == "mykey"
        assert s.s3_secret_key == "mysecret"
        assert s.s3_bucket_uploads == "my-bucket"
        assert s.ocr_enabled is True
        assert s.metrics_port == 9999

    def test_ocr_disabled(self):
        s = Settings(ocr_enabled=False)
        assert s.ocr_enabled is False

    def test_ocr_enabled(self):
        s = Settings(ocr_enabled=True)
        assert s.ocr_enabled is True
