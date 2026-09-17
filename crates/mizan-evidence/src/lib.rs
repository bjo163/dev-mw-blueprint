use mizan_model::{
    AnalyticalFactorsInput, CausalContributionLevel, ContextState, EvidenceRef, EvidenceReliability,
    ImpactLevel, IntentState, MizanInput, ReasonCode, ScopeLevel,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    None,
    Weak,
    Moderate,
    Strong,
    Verified,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceAssessment {
    pub total_evidence: usize,
    pub average_reliability: f64,
    pub strength: EvidenceStrength,
    pub provenance_complete: bool,
    pub known_factor_count: usize,
    pub bound_factor_count: usize,
    pub factor_coverage: f64,
    pub missing_evidence_ids: Vec<String>,
    pub sufficient_for_analytical_index: bool,
    pub reason_codes: Vec<ReasonCode>,
}

pub fn assess_evidence(input: &MizanInput) -> EvidenceAssessment {
    let average_reliability = average_reliability(&input.evidence);
    let strength = strength_from_score(average_reliability, input.evidence.is_empty());
    let provenance_complete = !input.evidence.is_empty()
        && input.evidence.iter().all(|e| !e.provenance.is_empty());

    let evidence_ids: HashSet<&str> = input.evidence.iter().map(|e| e.id.as_str()).collect();
    let factors = &input.analytical_factors;
    let known_factor_count = known_factor_count(factors);

    let bindings = [
        (&factors.evidence_bindings.intent, intent_known(factors)),
        (&factors.evidence_bindings.impact, impact_known(factors)),
        (&factors.evidence_bindings.scope, scope_known(factors)),
        (&factors.evidence_bindings.context, context_known(factors)),
        (
            &factors.evidence_bindings.causal_contribution,
            causal_known(factors),
        ),
    ];

    let mut bound_factor_count = 0usize;
    let mut missing_evidence_ids = Vec::new();

    for (ids, factor_known) in bindings {
        if !factor_known {
            continue;
        }
        if !ids.is_empty() {
            bound_factor_count += 1;
        }
        for id in ids {
            if !evidence_ids.contains(id.as_str()) && !missing_evidence_ids.contains(id) {
                missing_evidence_ids.push(id.clone());
            }
        }
    }

    let factor_coverage = if known_factor_count == 0 {
        0.0
    } else {
        bound_factor_count as f64 / known_factor_count as f64
    };

    let sufficient_for_analytical_index = known_factor_count == 5
        && bound_factor_count == 5
        && missing_evidence_ids.is_empty()
        && average_reliability >= 0.60;

    let mut reason_codes = Vec::new();
    if input.evidence.is_empty() {
        reason_codes.push(ReasonCode("EVIDENCE_NONE".into()));
    } else {
        reason_codes.push(ReasonCode("EVIDENCE_PRESENT".into()));
    }
    if provenance_complete {
        reason_codes.push(ReasonCode("PROVENANCE_COMPLETE".into()));
    } else {
        reason_codes.push(ReasonCode("PROVENANCE_INCOMPLETE".into()));
    }
    if missing_evidence_ids.is_empty() {
        reason_codes.push(ReasonCode("FACTOR_EVIDENCE_BINDINGS_VALID".into()));
    } else {
        reason_codes.push(ReasonCode("FACTOR_EVIDENCE_BINDING_MISSING_REFERENCE".into()));
    }
    if sufficient_for_analytical_index {
        reason_codes.push(ReasonCode("EVIDENCE_SUFFICIENT_FOR_ARI".into()));
    } else {
        reason_codes.push(ReasonCode("EVIDENCE_NOT_SUFFICIENT_FOR_ARI".into()));
    }

    EvidenceAssessment {
        total_evidence: input.evidence.len(),
        average_reliability,
        strength,
        provenance_complete,
        known_factor_count,
        bound_factor_count,
        factor_coverage,
        missing_evidence_ids,
        sufficient_for_analytical_index,
        reason_codes,
    }
}

fn average_reliability(evidence: &[EvidenceRef]) -> f64 {
    if evidence.is_empty() {
        return 0.0;
    }
    let total: f64 = evidence
        .iter()
        .map(|e| {
            let base = match e.reliability {
                EvidenceReliability::Unknown => 0.25,
                EvidenceReliability::Low => 0.40,
                EvidenceReliability::Medium => 0.60,
                EvidenceReliability::High => 0.80,
                EvidenceReliability::Verified => 1.00,
            };
            (base + if e.provenance.is_empty() { 0.0 } else { 0.05 }).min(1.0)
        })
        .sum();
    total / evidence.len() as f64
}

fn strength_from_score(score: f64, empty: bool) -> EvidenceStrength {
    if empty {
        EvidenceStrength::None
    } else if score >= 0.95 {
        EvidenceStrength::Verified
    } else if score >= 0.75 {
        EvidenceStrength::Strong
    } else if score >= 0.55 {
        EvidenceStrength::Moderate
    } else {
        EvidenceStrength::Weak
    }
}

fn known_factor_count(f: &AnalyticalFactorsInput) -> usize {
    [
        intent_known(f),
        impact_known(f),
        scope_known(f),
        context_known(f),
        causal_known(f),
    ]
    .into_iter()
    .filter(|known| *known)
    .count()
}

fn intent_known(f: &AnalyticalFactorsInput) -> bool {
    f.intent != IntentState::Unknown
}
fn impact_known(f: &AnalyticalFactorsInput) -> bool {
    f.impact != ImpactLevel::Unknown
}
fn scope_known(f: &AnalyticalFactorsInput) -> bool {
    f.scope != ScopeLevel::Unknown
}
fn context_known(f: &AnalyticalFactorsInput) -> bool {
    f.context != ContextState::Unknown
}
fn causal_known(f: &AnalyticalFactorsInput) -> bool {
    f.causal_contribution != CausalContributionLevel::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{
        ActiveRole, ActivityType, AuthorityDimension, EvidenceKind, FactorEvidenceBindings,
        MandateSource, MissionType, ProvenanceStep, RelationshipDomain, RouteKind,
    };

    fn complete_input() -> MizanInput {
        MizanInput {
            actor_id: "actor".into(),
            active_role: ActiveRole::Human,
            relationship_domains: vec![RelationshipDomain::GeneralCivil],
            activities: vec![ActivityType::GeneralCivil],
            mission_types: vec![MissionType::GeneralCivil],
            mandate_sources: vec![MandateSource::Personal],
            requested_authority_dimensions: vec![AuthorityDimension::Counsel],
            route: RouteKind::GeneralCivil,
            mandate_active: false,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: false,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![EvidenceRef {
                id: "E1".into(),
                source: "record".into(),
                kind: EvidenceKind::Document,
                reliability: EvidenceReliability::Verified,
                provenance: vec![ProvenanceStep {
                    source: "system-a".into(),
                    method: "signed-export".into(),
                    reference: Some("sha256:abc".into()),
                }],
            }],
            analytical_factors: AnalyticalFactorsInput {
                intent: IntentState::Deliberate,
                impact: ImpactLevel::High,
                scope: ScopeLevel::Local,
                context: ContextState::Normal,
                causal_contribution: CausalContributionLevel::Direct,
                evidence_bindings: FactorEvidenceBindings {
                    intent: vec!["E1".into()],
                    impact: vec!["E1".into()],
                    scope: vec!["E1".into()],
                    context: vec!["E1".into()],
                    causal_contribution: vec!["E1".into()],
                },
            },
        }
    }

    #[test]
    fn complete_verified_binding_is_sufficient() {
        let assessment = assess_evidence(&complete_input());
        assert_eq!(assessment.known_factor_count, 5);
        assert_eq!(assessment.bound_factor_count, 5);
        assert!(assessment.sufficient_for_analytical_index);
        assert!(assessment.missing_evidence_ids.is_empty());
    }

    #[test]
    fn missing_binding_reference_is_reported() {
        let mut input = complete_input();
        input.analytical_factors.evidence_bindings.intent = vec!["MISSING".into()];
        let assessment = assess_evidence(&input);
        assert!(!assessment.sufficient_for_analytical_index);
        assert_eq!(assessment.missing_evidence_ids, vec!["MISSING".to_string()]);
    }
}
