//! Identity domain: user accounts, roles, and authentication.
//!
//! This crate is mounted by `sis-api` at `/api/v1/identity`.

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sis_core::Health;

/// Name used for this domain in URLs and health checks.
pub const DOMAIN: &str = "identity";

/// A person who can sign in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub display_name: String,
}

/// Creates a [`User`], trimming surrounding whitespace from each field.
pub fn new_user(username: &str, display_name: &str) -> User {
    User {
        username: username.trim().to_string(),
        display_name: display_name.trim().to_string(),
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
    fn new_user_trims_whitespace() {
        let value = new_user("  astudent ", " Avery Student  ");
        assert_eq!(value.username, "astudent");
        assert_eq!(value.display_name, "Avery Student");
    }
}
