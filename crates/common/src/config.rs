use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub bind_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub database_url: String,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env(service_name: &str, default_port: u16) -> Self {
        Self {
            service: ServiceConfig {
                name: service_name.to_owned(),
                bind_addr: env::var("BIND_ADDR")
                    .unwrap_or_else(|_| format!("0.0.0.0:{default_port}")),
            },
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://signalforge:signalforge@localhost:5432/signalforge".to_owned()
            }),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_owned()),
            kafka_brokers: env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| "localhost:9092".to_owned()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-me".to_owned()),
        }
    }
}
