use rdkafka::consumer::{Consumer, StreamConsumer};
use signalforge_common::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("rules-engine");
    let config = AppConfig::from_env("rules-engine", 0);

    let consumer: StreamConsumer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("group.id", "signalforge-rules-engine")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["sf.events.raw.v1"])?;
    tracing::info!(kafka.brokers = %config.kafka_brokers, "rules engine subscribed");

    tokio::signal::ctrl_c().await?;
    Ok(())
}
