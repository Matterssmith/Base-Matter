use anyhow::Result;
use tracing_subscriber::EnvFilter;

pub fn init_logger(debug: bool) -> Result<()> {
    let default_level = if debug { "debug" } else { "info" };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init()
        .map_err(|error| anyhow::anyhow!("Logger initialization failed: {error}"))?;

    Ok(())
}