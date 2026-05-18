use signalforge_common::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("incident-service");
    let config = AppConfig::from_env("incident-service", 50051);

    tracing::info!(
        bind.addr = %config.service.bind_addr,
        database.configured = !config.database_url.is_empty(),
        "incident service ready for grpc implementation"
    );

    tokio::signal::ctrl_c().await?;
    Ok(())
}
