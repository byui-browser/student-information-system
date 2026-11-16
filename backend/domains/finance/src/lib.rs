//! Finance domain: student accounts, charges, payments, and holds.
//!
//! This crate is mounted by `sis-api` at `/api/v1/finance`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "finance";

/// A student's financial account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub student_id: String,
    pub status: String,
}

/// Creates a [`Account`], trimming surrounding whitespace from each field.
pub fn new_account(student_id: &str, status: &str) -> Account {
    Account {
        student_id: student_id.trim().to_string(),
        status: status.trim().to_string(),
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
    fn new_account_trims_whitespace() {
        let value = new_account("  s-1001 ", " open  ");
        assert_eq!(value.student_id, "s-1001");
        assert_eq!(value.status, "open");
    }
}
