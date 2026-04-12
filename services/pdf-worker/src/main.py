import asyncio
import json
import signal
import sys

import nats
import structlog
from prometheus_client import start_http_server

from config import Settings
from processor import PdfProcessor
from telemetry import setup_telemetry

logger = structlog.get_logger()


async def main():
    settings = Settings()
    setup_telemetry(settings)
    
    logger.info("Starting PDF Worker", nats_url=settings.nats_url)
    
    # Start Prometheus metrics server
    start_http_server(settings.metrics_port)
    logger.info("Metrics server started", port=settings.metrics_port)
    
    # Connect to NATS
    nc = await nats.connect(settings.nats_url)
    js = nc.jetstream()
    logger.info("Connected to NATS")
    
    # Initialize processor
    processor = PdfProcessor(settings)
    
    # Subscribe to PDF parse jobs
    async def handle_job(msg):
        try:
            job = json.loads(msg.data.decode())
            logger.info("Received PDF parse job", job_id=job.get("job_id"))
            
            await processor.process(job)
            
            await msg.ack()
            logger.info("Job completed", job_id=job.get("job_id"))
        except Exception as e:
            logger.error("Job failed", error=str(e), job_id=job.get("job_id"))
            await msg.nak()
    
    # Create consumer
    consumer = await js.pull_subscribe(
        "jobs.pdf.parse",
        durable="pdf-worker",
        stream="JOBS"
    )
    
    logger.info("Subscribed to jobs.pdf.parse")
    
    # Handle graceful shutdown
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
    
    # Cleanup
    await nc.close()
    logger.info("PDF Worker shutdown complete")


if __name__ == "__main__":
    asyncio.run(main())
