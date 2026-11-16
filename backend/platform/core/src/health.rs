use serde::{Deserialize, Serialize};

/// Basic service status returned by health endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health {
    pub service: String,
    pub status: String,
    pub version: String,
}

/// Builds a healthy [`Health`] report for the named service.
pub fn health(service: impl Into<String>) -> Health {
    Health {
        service: service.into(),
        status: "ok".to_string(),
        version: crate::API_VERSION.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_reports_ok_with_service_name() {
        let report = health("catalog");
        assert_eq!(report.service, "catalog");
        assert_eq!(report.status, "ok");
        assert_eq!(report.version, crate::API_VERSION);
    }
}
