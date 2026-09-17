use mizan_model::{MizanEvent, MizanResult, ReasonCode};
use mizan_routing::validate_route;
use mizan_th::resolve_th;

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

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{
        ActiveRole, ActivityType, AuthorityDimension, MandateSource, MissionType,
        RelationshipDomain, RouteKind, StructuralLevel, ThClass,
    };

    #[test]
    fn family_corrective_guidance_is_activity_not_level() {
        let event = MizanEvent {
            actor_id: "family-child".into(),
            level: StructuralLevel::L6,
            active_role: ActiveRole::Human,
            relationship_domains: vec![RelationshipDomain::FamilyKinship],
            activities: vec![ActivityType::CorrectiveGuidance],
            mission_types: vec![MissionType::Guidance],
            mandate_sources: vec![MandateSource::Personal],
            authority_dimensions: vec![AuthorityDimension::Counsel],
            route: RouteKind::FamilyGuidance,
            mandate_active: false,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: true,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![],
        };

        let result = evaluate(&event);
        assert_eq!(result.level, StructuralLevel::L6);
        assert_eq!(result.th, ThClass::Functional5);
        assert!(result.routing.valid);
    }

    #[test]
    fn civil_execution_without_active_mandate_is_rejected() {
        let event = MizanEvent {
            actor_id: "actor".into(),
            level: StructuralLevel::L4,
            active_role: ActiveRole::Police,
            relationship_domains: vec![RelationshipDomain::Governance],
            activities: vec![ActivityType::LawEnforcement],
            mission_types: vec![MissionType::LawEnforcement],
            mandate_sources: vec![MandateSource::State],
            authority_dimensions: vec![AuthorityDimension::Enforcement],
            route: RouteKind::CivilGovernance,
            mandate_active: false,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: false,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![],
        };

        let result = evaluate(&event);
        assert!(!result.routing.valid);
    }
}
