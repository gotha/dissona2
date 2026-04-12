use crate::config::Settings;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init(_settings: &Settings) -> anyhow::Result<()> {
    // Set up tracing subscriber with JSON formatting
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // TODO: Add OpenTelemetry tracing when Tempo is configured
    // TODO: Add Prometheus metrics endpoint

    Ok(())
}
