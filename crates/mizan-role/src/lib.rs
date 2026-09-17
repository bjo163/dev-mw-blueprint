use mizan_model::{ActiveRole, MizanInput, ReasonCode, RouteKind, StructuralLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleResolution {
    pub level: StructuralLevel,
    pub reason_codes: Vec<ReasonCode>,
}

pub fn resolve_level(input: &MizanInput) -> RoleResolution {
    use ActiveRole::*;

    let (level, reason) = match &input.active_role {
        Rasul | Nabi => (StructuralLevel::L2, "DIVINE_COMMISSION_ROLE"),
        Ulama | ReligiousScholar => (StructuralLevel::L3, "RELIGIOUS_STEWARDSHIP_ROLE"),
        UlulAmri | GovernmentLeader | Judge | Prosecutor | Police | Military => {
            if input.mandate_active {
                (StructuralLevel::L4, "ACTIVE_SPECIAL_CIVIL_MANDATE")
            } else if input.route == RouteKind::GeneralCivil {
                (StructuralLevel::L6, "SPECIAL_CIVIL_ROLE_INACTIVE_GENERAL_CONTEXT")
            } else {
                (StructuralLevel::L5, "SPECIAL_CIVIL_ROLE_WITHOUT_ACTIVE_MANDATE")
            }
        }
        Advocate => {
            if input.mandate_active {
                (StructuralLevel::L4, "ACTIVE_LEGAL_REPRESENTATION_MANDATE")
            } else {
                (StructuralLevel::L5, "LEGAL_EXPERT_WITHOUT_ACTIVE_CASE_MANDATE")
            }
        }
        Doctor | Nurse | Scientist | Engineer | Teacher | Parent | Guardian | Caregiver => {
            (StructuralLevel::L5, "EXPERT_FUNCTIONAL_SOCIAL_ROLE")
        }
        Citizen | Human => (StructuralLevel::L6, "GENERAL_HUMAN_CIVIL_ROLE"),
        Other(_) => (StructuralLevel::L6, "UNMAPPED_ROLE_CONSERVATIVE_HUMAN_BASELINE"),
    };

    RoleResolution {
        level,
        reason_codes: vec![ReasonCode(reason.into())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{ActiveRole, ActivityType, MandateSource, MissionType, RelationshipDomain};

    fn input(role: ActiveRole, mandate_active: bool, route: RouteKind) -> MizanInput {
        MizanInput {
            actor_id: "x".into(),
            active_role: role,
            relationship_domains: vec![RelationshipDomain::GeneralCivil],
            activities: vec![ActivityType::GeneralCivil],
            mission_types: vec![MissionType::GeneralCivil],
            mandate_sources: vec![MandateSource::Personal],
            requested_authority_dimensions: vec![],
            route,
            mandate_active,
            emergency_state: false,
            expert_knowledge_active: false,
            functional_responsibility_active: false,
            passive_role: false,
            trace_contribution: None,
            evidence: vec![],
        }
    }

    #[test]
    fn police_without_active_mandate_can_resolve_as_general_civil() {
        let r = resolve_level(&input(ActiveRole::Police, false, RouteKind::GeneralCivil));
        assert_eq!(r.level, StructuralLevel::L6);
    }

    #[test]
    fn police_with_active_mandate_resolves_to_l4() {
        let r = resolve_level(&input(ActiveRole::Police, true, RouteKind::CivilGovernance));
        assert_eq!(r.level, StructuralLevel::L4);
    }

    #[test]
    fn emergency_does_not_change_doctor_structural_level() {
        let mut i = input(ActiveRole::Doctor, false, RouteKind::ProfessionalService);
        i.emergency_state = true;
        let r = resolve_level(&i);
        assert_eq!(r.level, StructuralLevel::L5);
    }
}
