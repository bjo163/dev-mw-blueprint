use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use mizan_authority::resolve_authority;
use mizan_engine::{evaluate_input, MizanCalculation};
use mizan_evidence::{assess_evidence, EvidenceAssessment};
use mizan_factors::{FactorAssessment, ARI_CALIBRATION_VERSION};
use mizan_model::{AuthorityDimension, MizanInput, ReasonCode, StructuralLevel, ThClass};
use mizan_role::resolve_level;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;
use uuid::Uuid;

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ONTOLOGY_VERSION: &str = "1.2";
pub const CONTRACT_VERSION: &str = "1.2.0";
const OPENAPI_YAML: &str = include_str!("../../../openapi/mizan-api.v1.yaml");

#[derive(Clone, Default)]
pub struct AppState {
    pool: Option<PgPool>,
}

impl AppState {
    pub fn without_database() -> Self {
        Self { pool: None }
    }

    pub async fn from_env() -> Result<Self, ApiInitError> {
        let Ok(database_url) = std::env::var("DATABASE_URL") else {
            return Ok(Self::without_database());
        };

        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&database_url)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool: Some(pool) })
    }

    pub fn persistence_enabled(&self) -> bool {
        self.pool.is_some()
    }

    async fn persist(
        &self,
        input: &MizanInput,
        calculation: &MizanCalculation,
    ) -> Result<Option<Uuid>, ApiError> {
        let Some(pool) = &self.pool else {
            return Ok(None);
        };

        let record_id = Uuid::new_v4();
        let occurred_at = Utc::now();
        let input_json = serde_json::to_value(input)?;
        let result_json = serde_json::to_value(calculation)?;
        let input_hash = hash_json(&input_json)?;
        let result_hash = hash_json(&result_json)?;
        let evidence_strength = format!("{:?}", calculation.evidence_assessment.strength);
        let ari_value = calculation.factor_assessment.analytical_responsibility_index;
        let ari_calibration = calculation.factor_assessment.calibration_version.clone();

        sqlx::query(
            r#"
            INSERT INTO mizan_ledger (
                record_id, occurred_at, actor_id, input_json, result_json,
                input_hash, result_hash, ontology_version, contract_version,
                engine_version, evidence_strength, analytical_responsibility_index,
                ari_calibration_version, ari_evidence_sufficient
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
            "#,
        )
        .bind(record_id)
        .bind(occurred_at)
        .bind(&input.actor_id)
        .bind(input_json)
        .bind(result_json)
        .bind(input_hash)
        .bind(result_hash)
        .bind(ONTOLOGY_VERSION)
        .bind(CONTRACT_VERSION)
        .bind(ENGINE_VERSION)
        .bind(evidence_strength)
        .bind(ari_value)
        .bind(ari_calibration)
        .bind(calculation.analytical_index_evidence_sufficient)
        .execute(pool)
        .await?;

        Ok(Some(record_id))
    }
}

#[derive(Debug, Error)]
pub enum ApiInitError {
    #[error("database connection failed: {0}")]
    Database(#[from] sqlx::Error),
    #[error("database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("database operation failed: {0}")]
    Database(#[from] sqlx::Error),
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub engine: &'static str,
    pub engine_version: &'static str,
    pub ontology_version: &'static str,
    pub contract_version: &'static str,
    pub ari_calibration_version: &'static str,
    pub persistence_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct EvaluationResponse {
    pub record_id: Option<Uuid>,
    pub persisted: bool,
    pub calculation: MizanCalculation,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub level: StructuralLevel,
    pub reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Serialize)]
pub struct AuthorityResponse {
    pub valid: bool,
    pub resolved: Vec<AuthorityDimension>,
    pub denied: Vec<AuthorityDimension>,
    pub reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Serialize)]
pub struct ThResponse {
    pub level: StructuralLevel,
    pub th: ThClass,
    pub th_value: Option<f64>,
    pub reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Serialize)]
pub struct RouteResponse {
    pub route_valid: bool,
    pub authority_valid: bool,
    pub structurally_valid: bool,
    pub reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Serialize)]
pub struct EvidenceResponse {
    pub assessment: EvidenceAssessment,
}

#[derive(Debug, Serialize)]
pub struct FactorResponse {
    pub assessment: FactorAssessment,
    pub evidence_sufficient: bool,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/openapi.yaml", get(openapi))
        .route("/api/v1/analyze", post(analyze))
        .route("/api/v1/evaluate", post(evaluate))
        .route("/api/v1/resolve-role", post(resolve_role_handler))
        .route("/api/v1/resolve-authority", post(resolve_authority_handler))
        .route("/api/v1/resolve-th", post(resolve_th_handler))
        .route("/api/v1/validate-route", post(validate_route_handler))
        .route("/api/v1/assess-evidence", post(assess_evidence_handler))
        .route("/api/v1/score-factors", post(score_factors_handler))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "mizan-api",
        engine: "rust",
        engine_version: ENGINE_VERSION,
        ontology_version: ONTOLOGY_VERSION,
        contract_version: CONTRACT_VERSION,
        ari_calibration_version: ARI_CALIBRATION_VERSION,
        persistence_enabled: state.persistence_enabled(),
    })
}

async fn openapi() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/yaml; charset=utf-8")], OPENAPI_YAML)
}

async fn analyze(
    State(state): State<AppState>,
    Json(input): Json<MizanInput>,
) -> Result<Json<EvaluationResponse>, ApiError> {
    evaluate_common(state, input).await
}

async fn evaluate(
    State(state): State<AppState>,
    Json(input): Json<MizanInput>,
) -> Result<Json<EvaluationResponse>, ApiError> {
    evaluate_common(state, input).await
}

async fn evaluate_common(
    state: AppState,
    input: MizanInput,
) -> Result<Json<EvaluationResponse>, ApiError> {
    let calculation = evaluate_input(&input);
    let record_id = state.persist(&input, &calculation).await?;
    Ok(Json(EvaluationResponse {
        persisted: record_id.is_some(),
        record_id,
        calculation,
    }))
}

async fn resolve_role_handler(Json(input): Json<MizanInput>) -> Json<RoleResponse> {
    let resolution = resolve_level(&input);
    Json(RoleResponse {
        level: resolution.level,
        reason_codes: resolution.reason_codes,
    })
}

async fn resolve_authority_handler(Json(input): Json<MizanInput>) -> Json<AuthorityResponse> {
    let resolution = resolve_authority(&input);
    Json(AuthorityResponse {
        valid: resolution.valid,
        resolved: resolution.resolved,
        denied: resolution.denied,
        reason_codes: resolution.reason_codes,
    })
}

async fn resolve_th_handler(Json(input): Json<MizanInput>) -> Json<ThResponse> {
    let calculation = evaluate_input(&input);
    Json(ThResponse {
        level: calculation.resolved_event.level,
        th: calculation.result.th.clone(),
        th_value: calculation.th_value,
        reason_codes: calculation.reason_codes,
    })
}

async fn validate_route_handler(Json(input): Json<MizanInput>) -> Json<RouteResponse> {
    let calculation = evaluate_input(&input);
    Json(RouteResponse {
        route_valid: calculation.result.routing.valid,
        authority_valid: calculation.authority_valid,
        structurally_valid: calculation.structurally_valid,
        reason_codes: calculation.reason_codes,
    })
}

async fn assess_evidence_handler(Json(input): Json<MizanInput>) -> Json<EvidenceResponse> {
    Json(EvidenceResponse {
        assessment: assess_evidence(&input),
    })
}

async fn score_factors_handler(Json(input): Json<MizanInput>) -> Json<FactorResponse> {
    let calculation = evaluate_input(&input);
    Json(FactorResponse {
        assessment: calculation.factor_assessment,
        evidence_sufficient: calculation.analytical_index_evidence_sufficient,
    })
}

fn hash_json(value: &Value) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(hex::encode(digest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::Request,
    };
    use tower::ServiceExt;

    fn family_payload(extra_authority: Option<&str>) -> Value {
        let mut requested = vec![Value::String("Counsel".into())];
        if let Some(authority) = extra_authority {
            requested.push(Value::String(authority.into()));
        }
        serde_json::json!({
            "actor_id": "child",
            "active_role": "Human",
            "relationship_domains": ["FamilyKinship"],
            "activities": ["CorrectiveGuidance"],
            "mission_types": ["Guidance"],
            "mandate_sources": ["Personal"],
            "requested_authority_dimensions": requested,
            "route": "FamilyGuidance",
            "mandate_active": false,
            "emergency_state": false,
            "expert_knowledge_active": false,
            "functional_responsibility_active": true,
            "passive_role": false,
            "trace_contribution": null,
            "evidence": [{"id":"Q19:44","source":"Quran"}]
        })
    }

    fn analytical_payload() -> Value {
        serde_json::json!({
            "actor_id": "doctor-analytical",
            "active_role": "Doctor",
            "relationship_domains": ["Professional"],
            "activities": ["Healing"],
            "mission_types": ["Healing"],
            "mandate_sources": ["Professional"],
            "requested_authority_dimensions": ["Medical"],
            "route": "ProfessionalService",
            "mandate_active": false,
            "emergency_state": false,
            "expert_knowledge_active": true,
            "functional_responsibility_active": true,
            "passive_role": false,
            "trace_contribution": null,
            "evidence": [{
                "id":"E1",
                "source":"signed-record",
                "kind":"InstitutionalRecord",
                "reliability":"Verified",
                "provenance":[{
                    "source":"hospital-system",
                    "method":"signed-export",
                    "reference":"sha256:abc"
                }]
            }],
            "analytical_factors": {
                "intent":"Knowing",
                "impact":"Moderate",
                "scope":"Individual",
                "context":"ElevatedDuty",
                "causal_contribution":"Direct",
                "evidence_bindings": {
                    "intent":["E1"],
                    "impact":["E1"],
                    "scope":["E1"],
                    "context":["E1"],
                    "causal_contribution":["E1"]
                }
            }
        })
    }

    async fn post_json(uri: &str, body: Value) -> (StatusCode, Value) {
        let response = app(AppState::without_database())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value = serde_json::from_slice(&bytes).unwrap();
        (status, value)
    }

    #[tokio::test]
    async fn health_reports_no_db_in_test_state() {
        let response = app(AppState::without_database())
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["status"], "ok");
        assert_eq!(value["persistence_enabled"], false);
        assert_eq!(value["ontology_version"], "1.2");
        assert_eq!(value["ari_calibration_version"], "ARI-0.1.0");
    }

    #[tokio::test]
    async fn evaluate_resolves_family_corrective_without_trusting_level() {
        let (status, value) = post_json("/api/v1/evaluate", family_payload(None)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["calculation"]["resolved_event"]["level"], "L6");
        assert_eq!(value["calculation"]["th_value"], 5.0);
        assert_eq!(value["calculation"]["structurally_valid"], true);
        assert!(value["calculation"]["factor_assessment"]["analytical_responsibility_index"].is_null());
        assert_eq!(value["persisted"], false);
    }

    #[tokio::test]
    async fn api_rejects_authority_escalation_structurally() {
        let (status, value) = post_json("/api/v1/evaluate", family_payload(Some("Admin"))).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["calculation"]["authority_valid"], false);
        assert_eq!(value["calculation"]["structurally_valid"], false);
    }

    #[tokio::test]
    async fn role_endpoint_resolves_police_on_duty_to_l4() {
        let payload = serde_json::json!({
            "actor_id": "police",
            "active_role": "Police",
            "relationship_domains": ["Governance", "Legal"],
            "activities": ["LawEnforcement"],
            "mission_types": ["LawEnforcement", "Protection"],
            "mandate_sources": ["State", "Legal"],
            "requested_authority_dimensions": ["Enforcement", "Legal"],
            "route": "CivilGovernance",
            "mandate_active": true,
            "emergency_state": false,
            "expert_knowledge_active": false,
            "functional_responsibility_active": false,
            "passive_role": false,
            "trace_contribution": null,
            "evidence": [{"id":"badge","source":"state"}]
        });
        let (status, value) = post_json("/api/v1/resolve-role", payload).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["level"], "L4");
    }

    #[tokio::test]
    async fn evidence_endpoint_reports_verified_complete_provenance() {
        let (status, value) = post_json("/api/v1/assess-evidence", analytical_payload()).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["assessment"]["strength"], "Verified");
        assert_eq!(value["assessment"]["provenance_complete"], true);
        assert_eq!(value["assessment"]["sufficient_for_analytical_index"], true);
    }

    #[tokio::test]
    async fn factor_endpoint_returns_evidence_supported_ari() {
        let (status, value) = post_json("/api/v1/score-factors", analytical_payload()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(value["assessment"]["analytical_responsibility_index"].as_f64().is_some());
        assert_eq!(value["assessment"]["final_moral_or_divine_verdict"], false);
        assert_eq!(value["evidence_sufficient"], true);
    }
}
