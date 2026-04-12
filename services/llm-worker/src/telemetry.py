import structlog
from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

from config import Settings


def setup_telemetry(settings: Settings):
    """Set up structured logging and OpenTelemetry tracing."""

    # Structured logging
    structlog.configure(
        processors=[
            structlog.processors.TimeStamper(fmt="iso"),
            structlog.processors.add_log_level,
            structlog.processors.JSONRenderer(),
        ],
        wrapper_class=structlog.BoundLogger,
        context_class=dict,
        logger_factory=structlog.PrintLoggerFactory(),
    )

    # OpenTelemetry tracing
    resource = Resource.create({"service.name": "llm-worker"})
    provider = TracerProvider(resource=resource)

    try:
        otlp_exporter = OTLPSpanExporter(
            endpoint=settings.otlp_endpoint,
            insecure=True,
        )
        provider.add_span_processor(BatchSpanProcessor(otlp_exporter))
    except Exception as e:
        structlog.get_logger().warning("Failed to set up OTLP exporter", error=str(e))

    trace.set_tracer_provider(provider)
