//! Records domain: student profiles, grades, and academic history.
//!
//! This crate is mounted by `sis-api` at `/api/v1/records`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "records";

/// A student's core profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
}

/// Creates a [`Student`], trimming surrounding whitespace from each field.
pub fn new_student(id: &str, name: &str) -> Student {
    Student {
        id: id.trim().to_string(),
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
    fn new_student_trims_whitespace() {
        let value = new_student("  s-1001 ", " Avery Student  ");
        assert_eq!(value.id, "s-1001");
        assert_eq!(value.name, "Avery Student");
    }
}
