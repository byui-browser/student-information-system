//! Advising domain: degree programs, requirements, and degree audits.
//!
//! This crate is mounted by `sis-api` at `/api/v1/advising`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "advising";

/// A degree program, such as `BS Computer Science`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub code: String,
    pub name: String,
}

/// Creates a [`Program`], trimming surrounding whitespace from each field.
pub fn new_program(code: &str, name: &str) -> Program {
    Program {
        code: code.trim().to_string(),
        name: name.trim().to_string(),
    }
}

/// HTTP routes for this domain.
pub fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<Health> {
    Json(sis_core::health(DOMAIN))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_program_trims_whitespace() {
        let value = new_program("  BS-CS ", " BS Computer Science  ");
        assert_eq!(value.code, "BS-CS");
        assert_eq!(value.name, "BS Computer Science");
    }
}
