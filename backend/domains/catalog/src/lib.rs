//! Catalog domain: courses and the class sections offered each term.
//!
//! This crate is mounted by `sis-api` at `/api/v1/catalog`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "catalog";

/// A course in the catalog, such as `CS 101`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub code: String,
    pub title: String,
}

/// Creates a [`Course`], trimming surrounding whitespace from each field.
pub fn new_course(code: &str, title: &str) -> Course {
    Course {
        code: code.trim().to_string(),
        title: title.trim().to_string(),
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
    fn new_course_trims_whitespace() {
        let value = new_course("  CS 101 ", " Introduction to Programming  ");
        assert_eq!(value.code, "CS 101");
        assert_eq!(value.title, "Introduction to Programming");
    }
}
