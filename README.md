# SignalForge

SignalForge is a portfolio-grade realtime incident and observability platform.

## Stack

- Rust workspace with Axum services, Tonic-ready incident service, SQLx, Redis, and rdkafka.
- Next.js dashboard in `apps/web`.
- Docker Compose for Redpanda, Postgres, Redis, API gateway, and web.

## Current Status

This is the first implementation scaffold. API handlers return demo data, worker services subscribe to their planned Kafka topics, and persistence/gRPC internals are ready for the next pass.

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

## Planned Demo Flow

1. Login at `/login`.
2. Create an alert rule from `/rules`.
3. Submit an event to `POST /api/ingest/events`.
4. Watch `/events` update from the SSE stream.
5. Verify a matching rule opens an incident in `/incidents`.
