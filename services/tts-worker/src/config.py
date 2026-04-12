from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    # NATS
    nats_url: str = "nats://localhost:4222"
    
    # S3 Storage
    s3_endpoint: str = "http://localhost:9000"
    s3_access_key: str = "minioadmin"
    s3_secret_key: str = "minioadmin"
    s3_bucket_audio: str = "disona-audio"
    
    # TTS Provider
    tts_provider: str = "kokoro"  # kokoro, elevenlabs
    
    # Kokoro
    kokoro_model_path: str = "./models/kokoro-v0_19.onnx"
    
    # ElevenLabs (production)
    elevenlabs_api_key: str = ""
    
    # Audio settings
    audio_format: str = "aac"
    audio_bitrate: str = "128k"
    audio_sample_rate: int = 24000
    
    # Observability
    metrics_port: int = 9090
    otlp_endpoint: str = "http://localhost:4317"
    
    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
