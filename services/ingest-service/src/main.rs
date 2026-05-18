use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use rdkafka::producer::FutureProducer;
use signalforge_common::{AppConfig, dto::IngestEventRequest};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("ingest-service");
    let config = AppConfig::from_env("ingest-service", 8081);
    let addr: SocketAddr = config.service.bind_addr.parse()?;

    let _producer: FutureProducer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .create()?;

    let app = Router::new()
        .route("/events", post(ingest_event))
        .layer(TraceLayer::new_for_http());

    tracing::info!(%addr, kafka.brokers = %config.kafka_brokers, "ingest service listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn ingest_event(Json(payload): Json<IngestEventRequest>) -> Json<IngestEventRequest> {
    tracing::info!(service = %payload.service, severity = ?payload.severity, "accepted event");
    Json(payload)
}
