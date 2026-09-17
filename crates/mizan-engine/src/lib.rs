use mizan_authority::resolve_authority;
use mizan_model::{
    AuthorityDimension, MizanEvent, MizanInput, MizanResult, ReasonCode,
};
use mizan_role::resolve_level;
use mizan_routing::validate_route;
use mizan_th::resolve_th;
use serde::{Deserialize, Serialize};

/// Final deterministic engine output. This is a structural/responsibility
/// analysis result, not a divine or salvation judgment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MizanCalculation {
    pub resolved_event: MizanEvent,
    pub result: MizanResult,
    pub th_value: Option<f64>,
    pub authority_valid: bool,
    pub denied_authorities: Vec<AuthorityDimension>,
    pub structurally_valid: bool,
    pub evidence_sufficient: bool,
    pub reason_codes: Vec<ReasonCode>,
    pub final_divine_judgment_computed: bool,
}

/// Evaluate an already-resolved event. Kept for low-level/internal use.
pub fn evaluate(event: &MizanEvent) -> MizanResult {
    let th = resolve_th(event);
    let routing = validate_route(event);

    let mut reason_codes: Vec<ReasonCode> = th.reason_codes.clone();
    reason_codes.extend(routing.reason_codes.clone());

    MizanResult {
        level: event.level,
        th: th.th,
        routing,
        reason_codes,
    }
}

/// Canonical public pipeline: raw input -> resolved level -> resolved authority
/// -> TH -> routing -> structural Mizan calculation.
pub fn evaluate_input(input: &MizanInput) -> MizanCalculation {
    let role = resolve_level(input);
    let authority = resolve_authority(input);

    let event = MizanEvent {
        actor_id: input.actor_id.clone(),
        level: role.level,
        active_role: input.active_role.clone(),
        relationship_domains: input.relationship_domains.clone(),
        activities: input.activities.clone(),
        mission_types: input.mission_types.clone(),
        mandate_sources: input.mandate_sources.clone(),
        authority_dimensions: authority.resolved.clone(),
        route: input.route,
        mandate_active: input.mandate_active,
        emergency_state: input.emergency_state,
        expert_knowledge_active: input.expert_knowledge_active,
        functional_responsibility_active: input.functional_responsibility_active,
        passive_role: input.passive_role,
        trace_contribution: input.trace_contribution,
        evidence: input.evidence.clone(),
    };

    let result = evaluate(&event);
    let th_value = result.th.numeric_value();
    let evidence_sufficient = !event.evidence.is_empty();
    let structurally_valid = authority.valid && result.routing.valid;

    let mut reason_codes = role.reason_codes;
    reason_codes.extend(authority.reason_codes);
    reason_codes.extend(result.reason_codes.clone());
    reason_codes.push(ReasonCode(if evidence_sufficient {
        "EVIDENCE_PRESENT".into()
    } else {
        "EVIDENCE_NOT_PROVIDED".into()
    }));
    if structurally_valid {
        reason_codes.push(ReasonCode("STRUCTURAL_VALIDATION_PASSED".into()));
    } else {
        reason_codes.push(ReasonCode("STRUCTURAL_VALIDATION_FAILED".into()));
    }

    MizanCalculation {
        resolved_event: event,
        result,
        th_value,
        authority_valid: authority.valid,
        denied_authorities: authority.denied,
        structurally_valid,
        evidence_sufficient,
        reason_codes,
        final_divine_judgment_computed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{
        ActiveRole, ActivityType, AuthorityDimension, EvidenceRef, MandateSource, MissionType,
        RelationshipDomain, RouteKind, StructuralLevel, ThClass,
    };

    fn family_input() -> MizanInput {
        MizanInput {
            actor_id: "family-child".into(),
            active_role: ActiveRole::Human,
            relationship_domains: vec![RelationshipDomain::FamilyKinship],
            activities: vec![ActivityType::CorrectiveGuidance],
            mission_types: vec![MissionType::Guidance],
            mandate_sources: vec![MandateSource::Personal],
            requested_authority_dimensions: vec![AuthorityDimension::Counsel],
            route: RouteKind::FamilyGuidance,
            mandate_active: false,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: true,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![EvidenceRef { id: "Q19:44".into(), source: "Quran".into() }],
        }
    }

    #[test]
    fn family_corrective_guidance_is_activity_not_level() {
        let result = evaluate_input(&family_input());
        assert_eq!(result.resolved_event.level, StructuralLevel::L6);
        assert_eq!(result.result.th, ThClass::Functional5);
        assert_eq!(result.th_value, Some(5.0));
        assert!(result.result.routing.valid);
        assert!(result.authority_valid);
        assert!(result.structurally_valid);
        assert!(!result.final_divine_judgment_computed);
    }

    #[test]
    fn unauthorized_admin_request_is_rejected_without_removing_counsel() {
        let mut input = family_input();
        input.requested_authority_dimensions.push(AuthorityDimension::Admin);
        let result = evaluate_input(&input);
        assert!(!result.authority_valid);
        assert!(!result.structurally_valid);
        assert!(result.denied_authorities.contains(&AuthorityDimension::Admin));
        assert!(result.resolved_event.authority_dimensions.contains(&AuthorityDimension::Counsel));
        assert!(!result.resolved_event.authority_dimensions.contains(&AuthorityDimension::Admin));
    }

    #[test]
    fn police_on_duty_resolves_l4_and_th33() {
        let input = MizanInput {
            actor_id: "police-1".into(),
            active_role: ActiveRole::Police,
            relationship_domains: vec![RelationshipDomain::Governance, RelationshipDomain::Legal],
            activities: vec![ActivityType::LawEnforcement],
            mission_types: vec![MissionType::LawEnforcement, MissionType::Protection],
            mandate_sources: vec![MandateSource::State, MandateSource::Legal],
            requested_authority_dimensions: vec![AuthorityDimension::Enforcement, AuthorityDimension::Legal],
            route: RouteKind::CivilGovernance,
            mandate_active: true,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: false,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![EvidenceRef { id: "mandate-1".into(), source: "state".into() }],
        };
        let result = evaluate_input(&input);
        assert_eq!(result.resolved_event.level, StructuralLevel::L4);
        assert_eq!(result.result.th, ThClass::SpecialMission33);
        assert_eq!(result.th_value, Some(33.0));
        assert!(result.structurally_valid);
    }

    #[test]
    fn doctor_emergency_stays_l5_but_escalates_th() {
        let input = MizanInput {
            actor_id: "doctor-1".into(),
            active_role: ActiveRole::Doctor,
            relationship_domains: vec![RelationshipDomain::Professional],
            activities: vec![ActivityType::Healing],
            mission_types: vec![MissionType::Healing, MissionType::EmergencyResponse],
            mandate_sources: vec![MandateSource::Professional, MandateSource::Emergency],
            requested_authority_dimensions: vec![AuthorityDimension::Medical],
            route: RouteKind::ProfessionalService,
            mandate_active: true,
            emergency_state: true,
            expert_knowledge_active: true,
            functional_responsibility_active: true,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![EvidenceRef { id: "shift-1".into(), source: "hospital".into() }],
        };
        let result = evaluate_input(&input);
        assert_eq!(result.resolved_event.level, StructuralLevel::L5);
        assert_eq!(result.result.th, ThClass::SpecialMission33);
        assert_eq!(result.th_value, Some(33.0));
        assert!(result.structurally_valid);
    }
}
