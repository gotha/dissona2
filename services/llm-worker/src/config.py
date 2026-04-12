from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    # NATS
    nats_url: str = "nats://localhost:4222"

    # LLM via LiteLLM proxy
    litellm_url: str = "http://10.100.0.100:14000"
    llm_model: str = "qwen2.5:14b"

    # LLM Parameters
    llm_temperature: float = 0.3
    llm_max_tokens: int = 4096

    # Observability
    metrics_port: int = 9090
    otlp_endpoint: str = "http://localhost:4317"

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
