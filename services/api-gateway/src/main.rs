use std::{convert::Infallible, net::SocketAddr, time::Duration};

use axum::{
    Json, Router,
    extract::Path,
    response::{
        IntoResponse, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
    routing::{get, patch, post},
};
use chrono::Utc;
use signalforge_common::{
    AppConfig,
    dto::{
        AuthResponse, CreateRuleRequest, Event, Incident, IncidentStatus, IngestEventRequest,
        LoginRequest, RegisterRequest, Rule, Severity,
    },
};
use tokio_stream::{StreamExt, wrappers::IntervalStream};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("api-gateway");
    let config = AppConfig::from_env("api-gateway", 8080);
    let addr: SocketAddr = config.service.bind_addr.parse()?;

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/ingest/events", post(ingest_event))
        .route("/api/events/recent", get(recent_events))
        .route("/api/events/stream", get(event_stream))
        .route("/api/incidents", get(list_incidents))
        .route("/api/incidents/{id}", patch(update_incident))
        .route("/api/rules", get(list_rules).post(create_rule))
        .route("/api/rules/{id}", patch(update_rule))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing::info!(%addr, "api gateway listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn healthz() -> &'static str {
    "ok"
}

async fn register(Json(_payload): Json<RegisterRequest>) -> Json<AuthResponse> {
    Json(fake_auth_response())
}

async fn login(Json(_payload): Json<LoginRequest>) -> Json<AuthResponse> {
    Json(fake_auth_response())
}

async fn ingest_event(Json(payload): Json<IngestEventRequest>) -> impl IntoResponse {
    Json(Event {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        source: payload.source,
        service: payload.service,
        severity: payload.severity,
        message: payload.message,
        attributes: payload.attributes,
        received_at: Utc::now(),
    })
}

async fn recent_events() -> Json<Vec<Event>> {
    Json(vec![sample_event("checkout-api", Severity::Warning)])
}

async fn event_stream() -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(3))).map(|_| {
        let payload = serde_json::to_string(&sample_event("payments", Severity::Critical))
            .unwrap_or_else(|_| "{}".to_owned());
        Ok(SseEvent::default().event("event.received").data(payload))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn list_incidents() -> Json<Vec<Incident>> {
    Json(vec![Incident {
        id: Uuid::new_v4(),
        title: "High error rate on payments".to_owned(),
        service: "payments".to_owned(),
        severity: Severity::Critical,
        status: IncidentStatus::Open,
        created_at: Utc::now(),
    }])
}

async fn update_incident(Path(id): Path<Uuid>) -> Json<Incident> {
    Json(Incident {
        id,
        title: "High error rate on payments".to_owned(),
        service: "payments".to_owned(),
        severity: Severity::Critical,
        status: IncidentStatus::Acknowledged,
        created_at: Utc::now(),
    })
}

async fn list_rules() -> Json<Vec<Rule>> {
    Json(vec![Rule {
        id: Uuid::new_v4(),
        name: "Critical payment events".to_owned(),
        service: Some("payments".to_owned()),
        min_severity: Severity::Critical,
        enabled: true,
    }])
}

async fn create_rule(Json(payload): Json<CreateRuleRequest>) -> Json<Rule> {
    Json(Rule {
        id: Uuid::new_v4(),
        name: payload.name,
        service: payload.service,
        min_severity: payload.min_severity,
        enabled: true,
    })
}

async fn update_rule(Path(id): Path<Uuid>) -> Json<Rule> {
    Json(Rule {
        id,
        name: "Critical payment events".to_owned(),
        service: Some("payments".to_owned()),
        min_severity: Severity::Critical,
        enabled: true,
    })
}

fn fake_auth_response() -> AuthResponse {
    AuthResponse {
        token: "dev.jwt.token".to_owned(),
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
    }
}

fn sample_event(service: &str, severity: Severity) -> Event {
    Event {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        source: "demo-agent".to_owned(),
        service: service.to_owned(),
        severity,
        message: "Latency threshold exceeded".to_owned(),
        attributes: serde_json::json!({ "region": "iad", "p95_ms": 1280 }),
        received_at: Utc::now(),
    }
}
