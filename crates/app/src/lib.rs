use anyhow::Result;
use settings::Settings;
use tokio_util::sync::CancellationToken;

pub mod settings;

pub async fn run(cancellable: CancellationToken, settings: Settings) -> Result<()> {
    println!("Sleeping for {} seconds", settings.sleep_seconds);
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(settings.sleep_seconds)) => {
            tracing::info!("Finished sleeping");
        }
        _ = cancellable.cancelled() => {
            tracing::info!("Cancelled early");
        }
    }
    Ok(())
}
