use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};

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
#[cfg(feature = "kafka")]
use rdkafka::producer::{FutureProducer, FutureRecord};
use redis::AsyncCommands;
use serde::Deserialize;
use signalforge_common::{
    AppConfig,
    dto::{
        AuthResponse, CreateRuleRequest, Event, Incident, IncidentStatus, IngestEventRequest,
        LoginRequest, RegisterRequest, Rule, Severity,
    },
};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use tokio::sync::{RwLock, broadcast};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

const EVENT_TOPIC: &str = "event.received";
const REDIS_EVENT_CHANNEL: &str = "sf.events.live.v1";
const DEV_TENANT_ID: Uuid = Uuid::from_u128(0x11111111111111111111111111111111);

#[derive(Clone)]
struct AppState {
    inner: Arc<RwLock<LiveState>>,
    events_tx: broadcast::Sender<Event>,
    db: Option<PgPool>,
    redis: Option<redis::Client>,
    #[cfg(feature = "kafka")]
    kafka: Option<FutureProducer>,
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
    let db = connect_postgres(&config.database_url).await;
    let redis = connect_redis(&config.redis_url).await;
    #[cfg(feature = "kafka")]
    let kafka = connect_kafka(&config.kafka_brokers);
    if let Some(db) = &db {
        ensure_dev_tenant(db).await?;
    }

    let state = AppState {
        inner: Arc::new(RwLock::new(LiveState::seed())),
        events_tx: events_tx.clone(),
        db,
        redis: redis.clone(),
        #[cfg(feature = "kafka")]
        kafka,
    };
    if let Some(redis) = redis {
        spawn_redis_subscriber(redis, events_tx);
    }

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
        tenant_id: DEV_TENANT_ID,
        source: payload.source,
        service: payload.service,
        severity: payload.severity,
        message: payload.message,
        attributes: payload.attributes,
        received_at: Utc::now(),
    };

    if let Some(db) = &state.db {
        insert_event(db, &event).await;
        let rules = list_rules_from_db(db).await.unwrap_or_default();
        for rule in rules
            .iter()
            .filter(|rule| rule.enabled && rule_matches(rule, &event))
        {
            let incident = Incident {
                id: Uuid::new_v4(),
                title: format!("{} matched {}", event.service, rule.name),
                service: event.service.clone(),
                severity: event.severity,
                status: IncidentStatus::Open,
                created_at: Utc::now(),
            };
            insert_incident(db, &incident, Some(rule.id)).await;
        }
    } else {
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

    publish_redis_event(&state, &event).await;
    publish_kafka_event(&state, &event).await;
    let _ = state.events_tx.send(event.clone());
    Json(event)
}

async fn recent_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    if let Some(db) = &state.db {
        if let Ok(events) = list_events_from_db(db).await {
            return Json(events);
        }
    }

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
    if let Some(db) = &state.db {
        if let Ok(incidents) = list_incidents_from_db(db).await {
            return Json(incidents);
        }
    }

    Json(state.inner.read().await.incidents.clone())
}

async fn update_incident(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateIncidentRequest>,
) -> Json<Incident> {
    if let Some(db) = &state.db {
        if let Ok(incident) = update_incident_in_db(db, id, payload.status).await {
            return Json(incident);
        }
    }

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
    if let Some(db) = &state.db {
        if let Ok(rules) = list_rules_from_db(db).await {
            return Json(rules);
        }
    }

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

    if let Some(db) = &state.db {
        insert_rule(db, &rule).await;
    } else {
        state.inner.write().await.rules.insert(0, rule.clone());
    }
    Json(rule)
}

async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRuleRequest>,
) -> Json<Rule> {
    if let Some(db) = &state.db {
        if let Ok(rule) = update_rule_in_db(db, id, &payload).await {
            return Json(rule);
        }
    }

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

async fn connect_postgres(database_url: &str) -> Option<PgPool> {
    match PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(2))
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("postgres storage enabled");
            Some(pool)
        }
        Err(error) => {
            tracing::warn!(%error, "postgres unavailable; using in-memory fallback");
            None
        }
    }
}

async fn connect_redis(redis_url: &str) -> Option<redis::Client> {
    match redis::Client::open(redis_url) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(_) => {
                tracing::info!("redis live fanout enabled");
                Some(client)
            }
            Err(error) => {
                tracing::warn!(%error, "redis unavailable; using local SSE broadcast only");
                None
            }
        },
        Err(error) => {
            tracing::warn!(%error, "invalid redis url; using local SSE broadcast only");
            None
        }
    }
}

async fn ensure_dev_tenant(db: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO tenants (id, name)
        VALUES ($1, 'SignalForge Demo')
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(DEV_TENANT_ID)
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO rules (id, tenant_id, name, service, min_severity, enabled)
        VALUES
            ('22222222-2222-2222-2222-222222222222', $1, 'Critical payment events', 'payments', 'critical', true),
            ('33333333-3333-3333-3333-333333333333', $1, 'Checkout latency', 'checkout-api', 'warning', true)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(DEV_TENANT_ID)
    .execute(db)
    .await?;

    Ok(())
}

async fn insert_event(db: &PgPool, event: &Event) {
    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO events (id, tenant_id, source, service, severity, message, attributes, received_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(event.id)
    .bind(event.tenant_id)
    .bind(&event.source)
    .bind(&event.service)
    .bind(severity_to_str(event.severity))
    .bind(&event.message)
    .bind(&event.attributes)
    .bind(event.received_at)
    .execute(db)
    .await
    {
        tracing::error!(%error, "failed to persist event");
    }
}

async fn list_events_from_db(db: &PgPool) -> anyhow::Result<Vec<Event>> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, source, service, severity, message, attributes, received_at
        FROM events
        WHERE tenant_id = $1
        ORDER BY received_at DESC
        LIMIT 100
        "#,
    )
    .bind(DEV_TENANT_ID)
    .fetch_all(db)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(Event {
                id: row.try_get("id")?,
                tenant_id: row.try_get("tenant_id")?,
                source: row.try_get("source")?,
                service: row.try_get("service")?,
                severity: severity_from_str(row.try_get::<String, _>("severity")?.as_str()),
                message: row.try_get("message")?,
                attributes: row.try_get("attributes")?,
                received_at: row.try_get("received_at")?,
            })
        })
        .collect()
}

async fn insert_incident(db: &PgPool, incident: &Incident, rule_id: Option<Uuid>) {
    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO incidents (id, tenant_id, rule_id, title, service, severity, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, now())
        "#,
    )
    .bind(incident.id)
    .bind(DEV_TENANT_ID)
    .bind(rule_id)
    .bind(&incident.title)
    .bind(&incident.service)
    .bind(severity_to_str(incident.severity))
    .bind(status_to_str(incident.status))
    .bind(incident.created_at)
    .execute(db)
    .await
    {
        tracing::error!(%error, "failed to persist incident");
    }
}

async fn list_incidents_from_db(db: &PgPool) -> anyhow::Result<Vec<Incident>> {
    let rows = sqlx::query(
        r#"
        SELECT id, title, service, severity, status, created_at
        FROM incidents
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .bind(DEV_TENANT_ID)
    .fetch_all(db)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(Incident {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                service: row.try_get("service")?,
                severity: severity_from_str(row.try_get::<String, _>("severity")?.as_str()),
                status: status_from_str(row.try_get::<String, _>("status")?.as_str()),
                created_at: row.try_get("created_at")?,
            })
        })
        .collect()
}

async fn update_incident_in_db(
    db: &PgPool,
    id: Uuid,
    status: IncidentStatus,
) -> anyhow::Result<Incident> {
    let row = sqlx::query(
        r#"
        UPDATE incidents
        SET status = $2, updated_at = now()
        WHERE id = $1 AND tenant_id = $3
        RETURNING id, title, service, severity, status, created_at
        "#,
    )
    .bind(id)
    .bind(status_to_str(status))
    .bind(DEV_TENANT_ID)
    .fetch_one(db)
    .await?;

    Ok(Incident {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        service: row.try_get("service")?,
        severity: severity_from_str(row.try_get::<String, _>("severity")?.as_str()),
        status: status_from_str(row.try_get::<String, _>("status")?.as_str()),
        created_at: row.try_get("created_at")?,
    })
}

async fn insert_rule(db: &PgPool, rule: &Rule) {
    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO rules (id, tenant_id, name, service, min_severity, enabled)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(rule.id)
    .bind(DEV_TENANT_ID)
    .bind(&rule.name)
    .bind(&rule.service)
    .bind(severity_to_str(rule.min_severity))
    .bind(rule.enabled)
    .execute(db)
    .await
    {
        tracing::error!(%error, "failed to persist rule");
    }
}

async fn list_rules_from_db(db: &PgPool) -> anyhow::Result<Vec<Rule>> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, service, min_severity, enabled
        FROM rules
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(DEV_TENANT_ID)
    .fetch_all(db)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(Rule {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                service: row.try_get("service")?,
                min_severity: severity_from_str(row.try_get::<String, _>("min_severity")?.as_str()),
                enabled: row.try_get("enabled")?,
            })
        })
        .collect()
}

async fn update_rule_in_db(
    db: &PgPool,
    id: Uuid,
    payload: &UpdateRuleRequest,
) -> anyhow::Result<Rule> {
    let current = sqlx::query(
        r#"
        SELECT name, service, min_severity, enabled
        FROM rules
        WHERE id = $1 AND tenant_id = $2
        "#,
    )
    .bind(id)
    .bind(DEV_TENANT_ID)
    .fetch_one(db)
    .await?;

    let name = payload
        .name
        .clone()
        .unwrap_or_else(|| current.get::<String, _>("name"));
    let service = payload
        .service
        .clone()
        .or_else(|| current.get::<Option<String>, _>("service"));
    let min_severity = payload
        .min_severity
        .unwrap_or_else(|| severity_from_str(current.get::<String, _>("min_severity").as_str()));
    let enabled = payload
        .enabled
        .unwrap_or_else(|| current.get::<bool, _>("enabled"));

    let row = sqlx::query(
        r#"
        UPDATE rules
        SET name = $3, service = $4, min_severity = $5, enabled = $6
        WHERE id = $1 AND tenant_id = $2
        RETURNING id, name, service, min_severity, enabled
        "#,
    )
    .bind(id)
    .bind(DEV_TENANT_ID)
    .bind(name)
    .bind(service)
    .bind(severity_to_str(min_severity))
    .bind(enabled)
    .fetch_one(db)
    .await?;

    Ok(Rule {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        service: row.try_get("service")?,
        min_severity: severity_from_str(row.try_get::<String, _>("min_severity")?.as_str()),
        enabled: row.try_get("enabled")?,
    })
}

async fn publish_redis_event(state: &AppState, event: &Event) {
    let Some(client) = &state.redis else {
        return;
    };

    let payload = match serde_json::to_string(event) {
        Ok(payload) => payload,
        Err(error) => {
            tracing::error!(%error, "failed to encode redis event");
            return;
        }
    };

    match client.get_multiplexed_async_connection().await {
        Ok(mut connection) => {
            let result: redis::RedisResult<()> =
                connection.publish(REDIS_EVENT_CHANNEL, payload).await;
            if let Err(error) = result {
                tracing::error!(%error, "failed to publish redis event");
            }
        }
        Err(error) => tracing::error!(%error, "failed to connect to redis for publish"),
    }
}

fn spawn_redis_subscriber(client: redis::Client, events_tx: broadcast::Sender<Event>) {
    tokio::spawn(async move {
        let mut pubsub = match client.get_async_pubsub().await {
            Ok(pubsub) => pubsub,
            Err(error) => {
                tracing::error!(%error, "failed to start redis subscriber");
                return;
            }
        };

        if let Err(error) = pubsub.subscribe(REDIS_EVENT_CHANNEL).await {
            tracing::error!(%error, "failed to subscribe to redis event channel");
            return;
        }

        let mut stream = pubsub.on_message();
        while let Some(message) = stream.next().await {
            let payload: redis::RedisResult<String> = message.get_payload();
            match payload
                .ok()
                .and_then(|payload| serde_json::from_str::<Event>(&payload).ok())
            {
                Some(event) => {
                    let _ = events_tx.send(event);
                }
                None => tracing::warn!("ignored invalid redis event payload"),
            }
        }
    });
}

#[cfg(feature = "kafka")]
fn connect_kafka(brokers: &str) -> Option<FutureProducer> {
    match rdkafka::ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
    {
        Ok(producer) => {
            tracing::info!(kafka.brokers = %brokers, "kafka producer enabled");
            Some(producer)
        }
        Err(error) => {
            tracing::warn!(%error, kafka.brokers = %brokers, "kafka unavailable; skipping publish");
            None
        }
    }
}

#[cfg(feature = "kafka")]
async fn publish_kafka_event(state: &AppState, event: &Event) {
    let Some(producer) = &state.kafka else {
        return;
    };
    let payload = match serde_json::to_string(event) {
        Ok(payload) => payload,
        Err(error) => {
            tracing::error!(%error, "failed to encode kafka event");
            return;
        }
    };

    let record = FutureRecord::to("sf.events.raw.v1")
        .key(&event.id.to_string())
        .payload(&payload);

    if let Err((error, _)) = producer.send(record, Duration::from_secs(0)).await {
        tracing::error!(%error, "failed to publish kafka event");
    }
}

#[cfg(not(feature = "kafka"))]
async fn publish_kafka_event(_state: &AppState, _event: &Event) {}

fn severity_to_str(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Critical => "critical",
    }
}

fn severity_from_str(value: &str) -> Severity {
    match value {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    }
}

fn status_to_str(status: IncidentStatus) -> &'static str {
    match status {
        IncidentStatus::Open => "open",
        IncidentStatus::Acknowledged => "acknowledged",
        IncidentStatus::Resolved => "resolved",
    }
}

fn status_from_str(value: &str) -> IncidentStatus {
    match value {
        "resolved" => IncidentStatus::Resolved,
        "acknowledged" => IncidentStatus::Acknowledged,
        _ => IncidentStatus::Open,
    }
}
