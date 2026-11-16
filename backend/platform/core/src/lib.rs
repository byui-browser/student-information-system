//! Shared building blocks for every backend domain.
//!
//! Anything in here is used by more than one domain, so changes should be
//! small, backwards compatible, and reviewed by the platform owners.

mod health;

pub use health::{Health, health};

/// The version of the API, taken from the crate version at compile time.
pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");
