//! Admissions domain: applications and admission decisions.
//!
//! This crate is mounted by `sis-api` at `/api/v1/admissions`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "admissions";

/// An application for admission to a program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Application {
    pub applicant_name: String,
    pub program_code: String,
}

/// Creates a [`Application`], trimming surrounding whitespace from each field.
pub fn new_application(applicant_name: &str, program_code: &str) -> Application {
    Application {
        applicant_name: applicant_name.trim().to_string(),
        program_code: program_code.trim().to_string(),
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
    fn new_application_trims_whitespace() {
        let value = new_application("  Casey Morgan ", " BS-CS  ");
        assert_eq!(value.applicant_name, "Casey Morgan");
        assert_eq!(value.program_code, "BS-CS");
    }
}
