use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{
        IntoResponse, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
    routing::{get, patch, post},
};
use chrono::Utc;
use serde::Deserialize;
use signalforge_common::{
    AppConfig,
    dto::{
        AuthResponse, CreateRuleRequest, Event, Incident, IncidentStatus, IngestEventRequest,
        LoginRequest, RegisterRequest, Rule, Severity,
    },
};
use tokio::sync::{RwLock, broadcast};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

const EVENT_TOPIC: &str = "event.received";

#[derive(Clone)]
struct AppState {
    inner: Arc<RwLock<LiveState>>,
    events_tx: broadcast::Sender<Event>,
}

struct LiveState {
    events: Vec<Event>,
    incidents: Vec<Incident>,
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
struct UpdateIncidentRequest {
    status: IncidentStatus,
}

#[derive(Debug, Deserialize)]
struct UpdateRuleRequest {
    name: Option<String>,
    service: Option<String>,
    min_severity: Option<Severity>,
    enabled: Option<bool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    signalforge_telemetry::init("api-gateway");
    let config = AppConfig::from_env("api-gateway", 8080);
    let addr: SocketAddr = config.service.bind_addr.parse()?;
    let (events_tx, _) = broadcast::channel(256);
    let state = AppState {
        inner: Arc::new(RwLock::new(LiveState::seed())),
        events_tx,
    };

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
        .with_state(state)
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

async fn ingest_event(
    State(state): State<AppState>,
    Json(payload): Json<IngestEventRequest>,
) -> impl IntoResponse {
    let event = Event {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        source: payload.source,
        service: payload.service,
        severity: payload.severity,
        message: payload.message,
        attributes: payload.attributes,
        received_at: Utc::now(),
    };

    {
        let mut live = state.inner.write().await;
        let matching_rules = live
            .rules
            .iter()
            .filter(|rule| rule.enabled && rule_matches(rule, &event))
            .cloned()
            .collect::<Vec<_>>();

        for rule in matching_rules {
            live.incidents.insert(
                0,
                Incident {
                    id: Uuid::new_v4(),
                    title: format!("{} matched {}", event.service, rule.name),
                    service: event.service.clone(),
                    severity: event.severity,
                    status: IncidentStatus::Open,
                    created_at: Utc::now(),
                },
            );
        }

        live.events.insert(0, event.clone());
        live.events.truncate(100);
    }

    let _ = state.events_tx.send(event.clone());
    Json(event)
}

async fn recent_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    Json(state.inner.read().await.events.clone())
}

async fn event_stream(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let stream = BroadcastStream::new(state.events_tx.subscribe()).filter_map(|result| {
        result.ok().map(|event| {
            let payload = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_owned());
            Ok(SseEvent::default().event(EVENT_TOPIC).data(payload))
        })
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn list_incidents(State(state): State<AppState>) -> Json<Vec<Incident>> {
    Json(state.inner.read().await.incidents.clone())
}

async fn update_incident(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateIncidentRequest>,
) -> Json<Incident> {
    let mut live = state.inner.write().await;
    if let Some(incident) = live.incidents.iter_mut().find(|incident| incident.id == id) {
        incident.status = payload.status;
        return Json(incident.clone());
    }

    Json(Incident {
        id,
        title: "Unknown incident".to_owned(),
        service: "unknown".to_owned(),
        severity: Severity::Warning,
        status: payload.status,
        created_at: Utc::now(),
    })
}

async fn list_rules(State(state): State<AppState>) -> Json<Vec<Rule>> {
    Json(state.inner.read().await.rules.clone())
}

async fn create_rule(
    State(state): State<AppState>,
    Json(payload): Json<CreateRuleRequest>,
) -> Json<Rule> {
    let rule = Rule {
        id: Uuid::new_v4(),
        name: payload.name,
        service: payload.service,
        min_severity: payload.min_severity,
        enabled: true,
    };

    state.inner.write().await.rules.insert(0, rule.clone());
    Json(rule)
}

async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRuleRequest>,
) -> Json<Rule> {
    let mut live = state.inner.write().await;
    if let Some(rule) = live.rules.iter_mut().find(|rule| rule.id == id) {
        if let Some(name) = payload.name {
            rule.name = name;
        }
        if payload.service.is_some() {
            rule.service = payload.service;
        }
        if let Some(min_severity) = payload.min_severity {
            rule.min_severity = min_severity;
        }
        if let Some(enabled) = payload.enabled {
            rule.enabled = enabled;
        }
        return Json(rule.clone());
    }

    let rule = Rule {
        id,
        name: payload.name.unwrap_or_else(|| "Ad-hoc rule".to_owned()),
        service: payload.service,
        min_severity: payload.min_severity.unwrap_or(Severity::Warning),
        enabled: payload.enabled.unwrap_or(true),
    };
    live.rules.insert(0, rule.clone());
    Json(rule)
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

impl LiveState {
    fn seed() -> Self {
        Self {
            events: vec![
                sample_event("payments", Severity::Critical),
                sample_event("checkout-api", Severity::Warning),
                sample_event("webhook-worker", Severity::Info),
            ],
            incidents: vec![
                Incident {
                    id: Uuid::new_v4(),
                    title: "Payments elevated failures".to_owned(),
                    service: "payments".to_owned(),
                    severity: Severity::Critical,
                    status: IncidentStatus::Open,
                    created_at: Utc::now(),
                },
                Incident {
                    id: Uuid::new_v4(),
                    title: "Checkout latency spike".to_owned(),
                    service: "checkout-api".to_owned(),
                    severity: Severity::Warning,
                    status: IncidentStatus::Acknowledged,
                    created_at: Utc::now(),
                },
            ],
            rules: vec![
                Rule {
                    id: Uuid::new_v4(),
                    name: "Critical payment events".to_owned(),
                    service: Some("payments".to_owned()),
                    min_severity: Severity::Critical,
                    enabled: true,
                },
                Rule {
                    id: Uuid::new_v4(),
                    name: "Checkout latency".to_owned(),
                    service: Some("checkout-api".to_owned()),
                    min_severity: Severity::Warning,
                    enabled: true,
                },
            ],
        }
    }
}

fn rule_matches(rule: &Rule, event: &Event) -> bool {
    let service_matches = rule
        .service
        .as_ref()
        .is_none_or(|service| service == &event.service);
    service_matches && severity_rank(event.severity) >= severity_rank(rule.min_severity)
}

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::Info => 1,
        Severity::Warning => 2,
        Severity::Critical => 3,
    }
}
