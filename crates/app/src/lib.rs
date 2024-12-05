use anyhow::Result;
use tokio_util::sync::CancellationToken;

pub async fn run(ctx: CancellationToken) -> Result<()> {
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(120)) => {
            tracing::info!("Finished sleeping");
        }
        _ = ctx.cancelled() => {
            tracing::info!("Cancelled early");
        }
    }
    Ok(())
}
