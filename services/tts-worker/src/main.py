import asyncio
import json
import signal

import nats
import structlog
from prometheus_client import start_http_server

from config import Settings
from processor import TtsProcessor
from telemetry import setup_telemetry

logger = structlog.get_logger()


async def main():
    settings = Settings()
    setup_telemetry(settings)
    
    logger.info("Starting TTS Worker", provider=settings.tts_provider)
    
    # Start Prometheus metrics server
    start_http_server(settings.metrics_port)
    logger.info("Metrics server started", port=settings.metrics_port)
    
    # Connect to NATS
    nc = await nats.connect(settings.nats_url)
    js = nc.jetstream()
    logger.info("Connected to NATS")
    
    # Initialize processor
    processor = TtsProcessor(settings)
    
    async def handle_job(msg):
        job = json.loads(msg.data.decode())
        job_id = job.get("job_id")
        logger.info("Received TTS job", job_id=job_id)
        
        try:
            result = await processor.generate(job)
            
            # Publish completion event
            await js.publish(
                "events.tts.completed",
                json.dumps(result).encode()
            )
            
            await msg.ack()
            logger.info("TTS job completed", job_id=job_id, duration_ms=result.get("duration_ms"))
        except Exception as e:
            logger.error("TTS job failed", job_id=job_id, error=str(e))
            await msg.nak()
    
    # Subscribe to TTS jobs
    consumer = await js.pull_subscribe(
        "jobs.tts.generate",
        durable="tts-worker",
        stream="JOBS"
    )
    
    logger.info("Subscribed to jobs.tts.generate")
    
    # Graceful shutdown
    shutdown_event = asyncio.Event()
    
    def handle_shutdown(signum, frame):
        logger.info("Shutdown signal received")
        shutdown_event.set()
    
    signal.signal(signal.SIGTERM, handle_shutdown)
    signal.signal(signal.SIGINT, handle_shutdown)
    
    # Process jobs
    while not shutdown_event.is_set():
        try:
            msgs = await consumer.fetch(batch=1, timeout=5)
            for msg in msgs:
                await handle_job(msg)
        except nats.errors.TimeoutError:
            continue
        except Exception as e:
            logger.error("Error fetching jobs", error=str(e))
            await asyncio.sleep(1)
    
    await nc.close()
    logger.info("TTS Worker shutdown complete")


if __name__ == "__main__":
    asyncio.run(main())
