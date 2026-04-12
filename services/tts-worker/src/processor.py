import io
import subprocess
import tempfile
import uuid
from abc import ABC, abstractmethod
from typing import Optional

import boto3
import numpy as np
import soundfile as sf
import structlog
from prometheus_client import Counter, Histogram

from config import Settings

logger = structlog.get_logger()

# Metrics
TTS_JOBS = Counter("tts_jobs_total", "Total TTS jobs", ["provider", "status"])
TTS_DURATION = Histogram("tts_generation_duration_seconds", "TTS generation time")
TTS_AUDIO_SECONDS = Counter("tts_audio_generated_seconds", "Total audio seconds generated")
TTS_CHARACTERS = Counter("tts_characters_total", "Total characters processed")


class TtsProvider(ABC):
    @abstractmethod
    async def synthesize(self, text: str, voice_id: str) -> tuple[np.ndarray, int]:
        """Synthesize text to audio. Returns (audio_data, sample_rate)."""
        pass


class KokoroProvider(TtsProvider):
    def __init__(self, model_path: str):
        self.model_path = model_path
        self._model = None
    
    def _load_model(self):
        if self._model is None:
            try:
                from kokoro_onnx import Kokoro
                self._model = Kokoro(self.model_path)
                logger.info("Loaded Kokoro model")
            except Exception as e:
                logger.error("Failed to load Kokoro model", error=str(e))
                raise
    
    async def synthesize(self, text: str, voice_id: str) -> tuple[np.ndarray, int]:
        self._load_model()
        samples, sample_rate = self._model.create(text, voice=voice_id)
        return samples, sample_rate


class TtsProcessor:
    def __init__(self, settings: Settings):
        self.settings = settings
        self.provider = self._create_provider()
        
        # S3 client
        self.s3 = boto3.client(
            "s3",
            endpoint_url=settings.s3_endpoint,
            aws_access_key_id=settings.s3_access_key,
            aws_secret_access_key=settings.s3_secret_key,
        )
    
    def _create_provider(self) -> TtsProvider:
        if self.settings.tts_provider == "kokoro":
            return KokoroProvider(self.settings.kokoro_model_path)
        else:
            raise ValueError(f"Unknown TTS provider: {self.settings.tts_provider}")
    
    async def generate(self, job: dict) -> dict:
        """Generate audio from text."""
        audio_file_id = job["audio_file_id"]
        text = job["text"]
        voice_id = job.get("voice_id", "af_bella")
        
        TTS_CHARACTERS.inc(len(text))
        
        with TTS_DURATION.time():
            # Synthesize audio
            audio_data, sample_rate = await self.provider.synthesize(text, voice_id)
            
            # Convert to AAC
            aac_data = self._convert_to_aac(audio_data, sample_rate)
            
            # Calculate duration
            duration_ms = int(len(audio_data) / sample_rate * 1000)
            TTS_AUDIO_SECONDS.inc(duration_ms / 1000)
        
        # Upload to S3
        file_path = f"audio/{uuid.uuid4()}.aac"
        self.s3.put_object(
            Bucket=self.settings.s3_bucket_audio,
            Key=file_path,
            Body=aac_data,
            ContentType="audio/aac",
        )
        
        TTS_JOBS.labels(provider=self.settings.tts_provider, status="success").inc()
        
        return {
            "audio_file_id": audio_file_id,
            "file_path": file_path,
            "duration_ms": duration_ms,
            "file_size_bytes": len(aac_data),
        }
    
    def _convert_to_aac(self, audio_data: np.ndarray, sample_rate: int) -> bytes:
        """Convert audio to AAC format using ffmpeg."""
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as wav_file:
            sf.write(wav_file.name, audio_data, sample_rate)
            wav_path = wav_file.name
        
        with tempfile.NamedTemporaryFile(suffix=".aac", delete=False) as aac_file:
            aac_path = aac_file.name
        
        try:
            subprocess.run([
                "ffmpeg", "-y",
                "-i", wav_path,
                "-c:a", "aac",
                "-b:a", self.settings.audio_bitrate,
                "-ar", str(self.settings.audio_sample_rate),
                aac_path
            ], check=True, capture_output=True)
            
            with open(aac_path, "rb") as f:
                return f.read()
        finally:
            import os
            os.unlink(wav_path)
            os.unlink(aac_path)
