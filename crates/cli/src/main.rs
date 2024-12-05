use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use cli::settings;
use config::Config;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

const APP_NAME: &str = "CLI";

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    tokio_console_enabled: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.tokio_console_enabled {
        console_subscriber::init();
        tracing::info!("Tokio console enabled");
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init()
            .expect("should be able to initialize the logger");
    }

    let mut cfg = Config::builder();
    if let Some(cfg_path) = cli.config {
        cfg = cfg.add_source(config::File::from(cfg_path));
    }
    let cfg = cfg
        .add_source(config::Environment::with_prefix(APP_NAME))
        .build()?
        .try_deserialize::<settings::Settings>()?;
    tracing::info!(?cfg, "Starting up");

    let ctx = CancellationToken::new();
    let sigterm_timeout = cfg.sigterm_timeout_seconds;
    let app_ctx = ctx.clone();

    let sigterm_task = tokio::task::Builder::new()
        .name("sigterm handler")
        .spawn(async move {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
            tracing::warn!("Received Ctrl-C, cancelling tasks");
            ctx.cancel();

            if let Some(timeout) = sigterm_timeout {
                tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
                tracing::warn!("Timed out shutting down");
            } else {
                std::future::pending::<()>().await
            }
        })?;

    tokio::select! {
        _ = sigterm_task => {
            bail!("didn't stop gracefully");
        }
        result = app::run(app_ctx) => {
            result
        }
    }
}
