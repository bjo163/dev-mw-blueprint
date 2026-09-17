use mizan_model::{MandateSource, MissionType, MizanEvent, ReasonCode, StructuralLevel, ThClass};

#[derive(Debug, Clone, PartialEq)]
pub struct ThResolution {
    pub th: ThClass,
    pub reason_codes: Vec<ReasonCode>,
}

pub fn resolve_th(event: &MizanEvent) -> ThResolution {
    let high_mandate = event.mandate_active
        && matches!(event.level, StructuralLevel::L2 | StructuralLevel::L3 | StructuralLevel::L4)
        && !event.mission_types.is_empty();

    let emergency = event.emergency_state
        || event.mandate_sources.contains(&MandateSource::Emergency)
        || event.mission_types.contains(&MissionType::EmergencyResponse);

    if high_mandate || emergency {
        return ThResolution {
            th: ThClass::SpecialMission33,
            reason_codes: vec![ReasonCode(if emergency {
                "EMERGENCY_OR_SPECIAL_MISSION_ACTIVE".into()
            } else {
                "HIGH_MANDATE_ACTIVE".into()
            })],
        };
    }

    if event.expert_knowledge_active {
        return ThResolution {
            th: ThClass::Expert17,
            reason_codes: vec![ReasonCode("EXPERT_KNOWLEDGE_RESPONSIBILITY".into())],
        };
    }

    if event.functional_responsibility_active {
        return ThResolution {
            th: ThClass::Functional5,
            reason_codes: vec![ReasonCode("FUNCTIONAL_SOCIAL_RESPONSIBILITY".into())],
        };
    }

    if event.passive_role {
        return ThResolution {
            th: ThClass::Passive1,
            reason_codes: vec![ReasonCode("PASSIVE_MINOR_ROLE".into())],
        };
    }

    if let Some(trace) = event.trace_contribution {
        if (0.0..1.0).contains(&trace) {
            return ThResolution {
                th: ThClass::Trace(trace),
                reason_codes: vec![ReasonCode("TRACE_CAUSAL_CONTRIBUTION".into())],
            };
        }
    }

    if matches!(event.level, StructuralLevel::L6) {
        return ThResolution {
            th: ThClass::HumanBaseline2_5,
            reason_codes: vec![ReasonCode("GENERAL_HUMAN_BASELINE".into())],
        };
    }

    ThResolution {
        th: ThClass::Unknown,
        reason_codes: vec![ReasonCode("TH_INSUFFICIENT_EVIDENCE".into())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{
        ActiveRole, AnalyticalFactorsInput, AuthorityDimension, RelationshipDomain, RouteKind,
    };

    fn base_event(level: StructuralLevel) -> MizanEvent {
        MizanEvent {
            actor_id: "test".into(),
            level,
            active_role: ActiveRole::Human,
            relationship_domains: vec![RelationshipDomain::GeneralCivil],
            activities: vec![],
            mission_types: vec![],
            mandate_sources: vec![],
            authority_dimensions: vec![AuthorityDimension::Social],
            route: RouteKind::GeneralCivil,
            mandate_active: false,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: false,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![],
            analytical_factors: AnalyticalFactorsInput::default(),
        }
    }

    #[test]
    fn l6_defaults_to_human_baseline() {
        let result = resolve_th(&base_event(StructuralLevel::L6));
        assert_eq!(result.th, ThClass::HumanBaseline2_5);
    }

    #[test]
    fn emergency_escalates_to_33_without_changing_level() {
        let mut event = base_event(StructuralLevel::L5);
        event.emergency_state = true;
        let result = resolve_th(&event);
        assert_eq!(result.th, ThClass::SpecialMission33);
        assert_eq!(event.level, StructuralLevel::L5);
    }
}
