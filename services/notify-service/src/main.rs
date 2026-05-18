use rdkafka::consumer::{Consumer, StreamConsumer};
use signalforge_common::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("notify-service");
    let config = AppConfig::from_env("notify-service", 0);

    let consumer: StreamConsumer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("group.id", "signalforge-notify-service")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["sf.notifications.requested.v1"])?;
    tracing::info!("notify service subscribed; webhook delivery is stubbed for MVP");

    tokio::signal::ctrl_c().await?;
    Ok(())
}
