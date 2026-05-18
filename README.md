# SignalForge

SignalForge is a portfolio-grade realtime incident and observability platform.

## Stack

- Rust workspace with Axum services, Tonic-ready incident service, SQLx, Redis, and rdkafka.
- Next.js dashboard in `apps/web`.
- Docker Compose for Redpanda, Postgres, Redis, API gateway, and web.

## Current Status

The dashboard talks to the API gateway over REST and SSE. The API gateway now prefers production backing services when available:

- Postgres stores events, alert rules, and incidents.
- Redis publishes live event fanout on `sf.events.live.v1` for SSE delivery.
- Kafka/Redpanda publishing to `sf.events.raw.v1` is enabled in Docker builds with the `kafka` feature.
- If Postgres or Redis are not running during local development, the gateway falls back to in-memory state so the UI still works.

## Local Notes

The project is pinned with `rust-toolchain.toml` to `1.93.0` because the plan requires Rust `1.93.0`.

Docker is required for the Compose stack and was not available on PATH during scaffolding.

## Commands

```powershell
cd D:\signalforge
npm install
npm --workspace @signalforge/web run dev
```

```powershell
cd D:\signalforge
cargo metadata --no-deps
```

When Docker is installed:

```powershell
cd D:\signalforge
docker compose up --build
```

Local API without Docker:

```powershell
cd D:\signalforge
$env:BIND_ADDR='127.0.0.1:8080'
cargo run -p api-gateway
```

## Planned Demo Flow

1. Login at `/login`.
2. Create an alert rule from `/rules`.
3. Submit an event to `POST /api/ingest/events`.
4. Watch `/events` update from the SSE stream.
5. Verify a matching rule opens an incident in `/incidents`.
