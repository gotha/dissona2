import asyncio
import json
import signal

import nats
import structlog
from prometheus_client import start_http_server

from config import Settings
from processor import LlmProcessor
from telemetry import setup_telemetry

logger = structlog.get_logger()


async def main():
    settings = Settings()
    setup_telemetry(settings)

    logger.info("Starting LLM Worker", model=settings.llm_model)

    # Start Prometheus metrics server
    start_http_server(settings.metrics_port)
    logger.info("Metrics server started", port=settings.metrics_port)

    # Connect to NATS
    nc = await nats.connect(settings.nats_url)
    js = nc.jetstream()
    logger.info("Connected to NATS")

    # Initialize processor
    processor = LlmProcessor(settings)

    # Job handlers
    async def handle_segment_job(msg):
        job = json.loads(msg.data.decode())
        logger.info("Received segment job", job_id=job.get("job_id"))
        try:
            result = await processor.segment_text(job)
            # Publish result event
            await js.publish("events.llm.segment.completed", json.dumps(result).encode())
            await msg.ack()
        except Exception as e:
            logger.error("Segment job failed", error=str(e))
            await msg.nak()

    async def handle_summarize_job(msg):
        job = json.loads(msg.data.decode())
        logger.info("Received summarize job", job_id=job.get("job_id"))
        try:
            result = await processor.summarize(job)
            await js.publish("events.llm.summarize.completed", json.dumps(result).encode())
            await msg.ack()
        except Exception as e:
            logger.error("Summarize job failed", error=str(e))
            await msg.nak()

    async def handle_keypoints_job(msg):
        job = json.loads(msg.data.decode())
        logger.info("Received keypoints job", job_id=job.get("job_id"))
        try:
            result = await processor.extract_keypoints(job)
            await js.publish("events.llm.keypoints.completed", json.dumps(result).encode())
            await msg.ack()
        except Exception as e:
            logger.error("Keypoints job failed", error=str(e))
            await msg.nak()

    # Subscribe to job queues
    segment_consumer = await js.pull_subscribe("jobs.llm.segment", "llm-segment", stream="JOBS")
    summarize_consumer = await js.pull_subscribe(
        "jobs.llm.summarize", "llm-summarize", stream="JOBS"
    )
    keypoints_consumer = await js.pull_subscribe(
        "jobs.llm.keypoints", "llm-keypoints", stream="JOBS"
    )

    logger.info("Subscribed to LLM job queues")

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
            # Process each queue
            for consumer, handler in [
                (segment_consumer, handle_segment_job),
                (summarize_consumer, handle_summarize_job),
                (keypoints_consumer, handle_keypoints_job),
            ]:
                try:
                    msgs = await consumer.fetch(batch=1, timeout=1)
                    for msg in msgs:
                        await handler(msg)
                except nats.errors.TimeoutError:
                    continue
        except Exception as e:
            logger.error("Error processing jobs", error=str(e))
            await asyncio.sleep(1)

    await nc.close()
    logger.info("LLM Worker shutdown complete")


if __name__ == "__main__":
    asyncio.run(main())
