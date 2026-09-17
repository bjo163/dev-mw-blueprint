use axum::{body::{to_bytes, Body}, http::{header, Request, StatusCode}};
use mizan_api::{app, AppState};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn evaluate_persists_to_postgres_when_database_url_is_configured() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }

    let state = AppState::from_env().await.expect("postgres + migrations available");
    assert!(state.persistence_enabled());

    let payload = serde_json::json!({
        "actor_id": "ci-postgres-human",
        "active_role": "Human",
        "relationship_domains": ["FamilyKinship"],
        "activities": ["CorrectiveGuidance"],
        "mission_types": ["Guidance"],
        "mandate_sources": ["Personal"],
        "requested_authority_dimensions": ["Counsel"],
        "route": "FamilyGuidance",
        "mandate_active": false,
        "emergency_state": false,
        "expert_knowledge_active": false,
        "functional_responsibility_active": true,
        "passive_role": false,
        "trace_contribution": null,
        "evidence": [{"id":"CI-PG-001","source":"integration-test"}]
    });

    let response = app(state)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/evaluate")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["persisted"], true);
    assert!(value["record_id"].as_str().is_some());
    assert_eq!(value["calculation"]["resolved_event"]["level"], "L6");
    assert_eq!(value["calculation"]["th_value"], 5.0);
}
