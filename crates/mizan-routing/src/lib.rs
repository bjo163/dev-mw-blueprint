use mizan_model::{
    ActivityType, AuthorityDimension, MizanEvent, ReasonCode, RouteKind, RoutingDecision,
};

pub fn validate_route(event: &MizanEvent) -> RoutingDecision {
    match event.route {
        RouteKind::MessageGuidance => RoutingDecision {
            valid: true,
            reason_codes: vec![ReasonCode("MESSAGE_GUIDANCE_ROUTE_VALID".into())],
        },
        RouteKind::ReligiousStewardship => RoutingDecision {
            valid: true,
            reason_codes: vec![ReasonCode("RELIGIOUS_STEWARDSHIP_ROUTE_VALID".into())],
        },
        RouteKind::CivilGovernance => validate_civil_governance(event),
        RouteKind::ProfessionalService => RoutingDecision {
            valid: true,
            reason_codes: vec![ReasonCode("PROFESSIONAL_SERVICE_ROUTE_VALID".into())],
        },
        RouteKind::FamilyGuidance | RouteKind::CrossLevelCounsel => validate_counsel(event),
        RouteKind::GeneralCivil => RoutingDecision {
            valid: true,
            reason_codes: vec![ReasonCode("GENERAL_CIVIL_ROUTE_VALID".into())],
        },
    }
}

fn validate_civil_governance(event: &MizanEvent) -> RoutingDecision {
    let has_admin_or_legal_authority = event
        .authority_dimensions
        .iter()
        .any(|a| matches!(a, AuthorityDimension::Admin | AuthorityDimension::Legal | AuthorityDimension::Enforcement));

    if !event.mandate_active || !has_admin_or_legal_authority {
        return RoutingDecision {
            valid: false,
            reason_codes: vec![ReasonCode("CIVIL_EXECUTION_REQUIRES_ACTIVE_CIVIL_MANDATE".into())],
        };
    }

    RoutingDecision {
        valid: true,
        reason_codes: vec![ReasonCode("CIVIL_MANDATE_ROUTE_VALID".into())],
    }
}

fn validate_counsel(event: &MizanEvent) -> RoutingDecision {
    let is_corrective = event.activities.iter().any(|a| {
        matches!(
            a,
            ActivityType::Nasihah | ActivityType::Guidance | ActivityType::CorrectiveGuidance
        )
    });
    let has_counsel = event
        .authority_dimensions
        .contains(&AuthorityDimension::Counsel);
    let borrows_admin_power = event
        .authority_dimensions
        .iter()
        .any(|a| matches!(a, AuthorityDimension::Admin | AuthorityDimension::Enforcement));

    if !is_corrective || !has_counsel {
        return RoutingDecision {
            valid: false,
            reason_codes: vec![ReasonCode("COUNSEL_ROUTE_REQUIRES_COUNSEL_ACTIVITY_AND_AUTHORITY".into())],
        };
    }

    if borrows_admin_power && !event.mandate_active {
        return RoutingDecision {
            valid: false,
            reason_codes: vec![ReasonCode("COUNSEL_MUST_NOT_BORROW_ADMIN_OR_ENFORCEMENT_AUTHORITY".into())],
        };
    }

    RoutingDecision {
        valid: true,
        reason_codes: vec![ReasonCode("CORRECTIVE_COUNSEL_ROUTE_VALID".into())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{
        ActiveRole, AnalyticalFactorsInput, MandateSource, MissionType, RelationshipDomain,
        StructuralLevel,
    };

    fn counsel_event() -> MizanEvent {
        MizanEvent {
            actor_id: "child".into(),
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
            analytical_factors: AnalyticalFactorsInput::default(),
        }
    }

    #[test]
    fn corrective_guidance_is_valid_without_admin_power() {
        let result = validate_route(&counsel_event());
        assert!(result.valid);
    }

    #[test]
    fn counsel_cannot_borrow_enforcement_power_without_mandate() {
        let mut event = counsel_event();
        event.authority_dimensions.push(AuthorityDimension::Enforcement);
        let result = validate_route(&event);
        assert!(!result.valid);
    }
}
