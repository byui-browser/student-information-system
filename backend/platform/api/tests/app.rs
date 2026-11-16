use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn get(uri: &str) -> (StatusCode, serde_json::Value) {
    let response = sis_api::app()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

#[tokio::test]
async fn root_health_is_ok() {
    let (status, body) = get("/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["service"], "api");
}

#[tokio::test]
async fn every_domain_is_mounted() {
    for domain in sis_api::DOMAINS {
        let (status, body) = get(&format!("{}/{domain}/health", sis_api::API_PREFIX)).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{domain} health should be reachable"
        );
        assert_eq!(body["service"], domain);
    }
}
