from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    # NATS
    nats_url: str = "nats://localhost:4222"

    # S3 Storage
    s3_endpoint: str = "http://localhost:9000"
    s3_access_key: str = "minioadmin"
    s3_secret_key: str = "minioadmin"
    s3_bucket_uploads: str = "disona-uploads"

    # LiteLLM (for embeddings fallback)
    litellm_url: str = "http://10.100.0.100:14000"

    # Observability
    metrics_port: int = 9090
    otlp_endpoint: str = "http://localhost:4317"

    # Processing
    max_pages: int = 1000
    ocr_enabled: bool = True

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
