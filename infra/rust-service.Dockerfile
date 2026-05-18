FROM rust:1.93-bookworm AS builder

ARG SERVICE
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake pkg-config libssl-dev protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release -p ${SERVICE}

FROM debian:bookworm-slim

ARG SERVICE
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/${SERVICE} /usr/local/bin/service
CMD ["service"]

