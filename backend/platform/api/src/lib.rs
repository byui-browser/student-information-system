//! Composes every domain into one HTTP application.
//!
//! Each domain owns its own router. This crate only decides where each one
//! is mounted and which cross-cutting layers (CORS, tracing) wrap them.
//! Adding a domain is a one-line change in [`DOMAINS`] and [`api_router`].

use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};
use sis_core::Health;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// URL prefix for every domain route.
pub const API_PREFIX: &str = "/api/v1";

/// Names of every mounted domain, in mount order.
pub const DOMAINS: [&str; 7] = [
    "identity",
    "catalog",
    "enrollment",
    "records",
    "finance",
    "advising",
    "admissions",
];

/// Runtime settings, read from environment variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Address to listen on. Env: `SIS_ADDR`. Default: `127.0.0.1:8080`.
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Builds config from any key lookup, so it can be tested without
    /// touching real environment variables.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Self {
        let addr = lookup("SIS_ADDR")
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 8080)));
        Self { addr }
    }
}

/// Every domain router, nested under its own path.
pub fn api_router() -> Router {
    Router::new()
        .nest("/identity", sis_identity::router())
        .nest("/catalog", sis_catalog::router())
        .nest("/enrollment", sis_enrollment::router())
        .nest("/records", sis_records::router())
        .nest("/finance", sis_finance::router())
        .nest("/advising", sis_advising::router())
        .nest("/admissions", sis_admissions::router())
}

/// The full application: health check, versioned API, and shared layers.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest(API_PREFIX, api_router())
        // Permissive CORS lets the web and desktop clients call the API
        // during development. Tighten this before deploying.
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<Health> {
    Json(sis_core::health("api"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_to_localhost_8080() {
        let config = Config::from_lookup(|_| None);
        assert_eq!(config.addr.to_string(), "127.0.0.1:8080");
    }
}
