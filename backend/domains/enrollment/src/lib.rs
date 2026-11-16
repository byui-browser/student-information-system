//! Enrollment domain: registering students into sections and managing waitlists.
//!
//! This crate is mounted by `sis-api` at `/api/v1/enrollment`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "enrollment";

/// A student's place in a section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enrollment {
    pub student_id: String,
    pub section_id: String,
}

/// Creates a [`Enrollment`], trimming surrounding whitespace from each field.
pub fn new_enrollment(student_id: &str, section_id: &str) -> Enrollment {
    Enrollment {
        student_id: student_id.trim().to_string(),
        section_id: section_id.trim().to_string(),
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
    fn new_enrollment_trims_whitespace() {
        let value = new_enrollment("  s-1001 ", " CS 101-01  ");
        assert_eq!(value.student_id, "s-1001");
        assert_eq!(value.section_id, "CS 101-01");
    }
}
