use mizan_model::{
    ActiveRole, ActivityType, AuthorityDimension, MandateSource, MizanInput, MissionType, ReasonCode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityResolution {
    pub valid: bool,
    pub resolved: Vec<AuthorityDimension>,
    pub denied: Vec<AuthorityDimension>,
    pub reason_codes: Vec<ReasonCode>,
}

pub fn resolve_authority(input: &MizanInput) -> AuthorityResolution {
    let mut resolved = Vec::new();
    let corrective = input.activities.iter().any(|a| {
        matches!(a, ActivityType::Nasihah | ActivityType::Guidance | ActivityType::CorrectiveGuidance)
    });

    match &input.active_role {
        ActiveRole::Rasul | ActiveRole::Nabi => {
            if input.activities.contains(&ActivityType::Balagh)
                || input.mission_types.contains(&MissionType::Balagh)
                || input.mission_types.contains(&MissionType::Revelation)
            {
                push_unique(&mut resolved, AuthorityDimension::Message);
            }
            if corrective || input.mission_types.contains(&MissionType::Guidance) {
                push_unique(&mut resolved, AuthorityDimension::Guidance);
            }
        }
        ActiveRole::Ulama | ActiveRole::ReligiousScholar => {
            if corrective
                || input.mission_types.contains(&MissionType::Guidance)
                || input.mission_types.contains(&MissionType::ReligiousStewardship)
            {
                push_unique(&mut resolved, AuthorityDimension::Guidance);
            }
            if input.activities.contains(&ActivityType::Education)
                || input.mission_types.contains(&MissionType::Education)
            {
                push_unique(&mut resolved, AuthorityDimension::Educational);
            }
        }
        ActiveRole::UlulAmri | ActiveRole::GovernmentLeader => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Admin);
                if input.mandate_sources.contains(&MandateSource::Legal) {
                    push_unique(&mut resolved, AuthorityDimension::Legal);
                }
            }
        }
        ActiveRole::Judge => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Legal);
            }
        }
        ActiveRole::Prosecutor => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Legal);
                if input.mission_types.contains(&MissionType::LawEnforcement) {
                    push_unique(&mut resolved, AuthorityDimension::Enforcement);
                }
            }
        }
        ActiveRole::Police => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Enforcement);
                if input.mandate_sources.contains(&MandateSource::Legal) {
                    push_unique(&mut resolved, AuthorityDimension::Legal);
                }
            }
        }
        ActiveRole::Military => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Enforcement);
            }
        }
        ActiveRole::Advocate => {
            if input.mandate_active {
                push_unique(&mut resolved, AuthorityDimension::Legal);
            }
        }
        ActiveRole::Doctor | ActiveRole::Nurse => {
            push_unique(&mut resolved, AuthorityDimension::Medical);
        }
        ActiveRole::Scientist | ActiveRole::Engineer => {
            push_unique(&mut resolved, AuthorityDimension::Technical);
        }
        ActiveRole::Teacher => {
            push_unique(&mut resolved, AuthorityDimension::Educational);
        }
        ActiveRole::Parent | ActiveRole::Guardian | ActiveRole::Caregiver => {
            push_unique(&mut resolved, AuthorityDimension::Social);
        }
        ActiveRole::Citizen | ActiveRole::Human | ActiveRole::Other(_) => {}
    }

    if corrective {
        push_unique(&mut resolved, AuthorityDimension::Counsel);
    }

    let denied: Vec<AuthorityDimension> = input
        .requested_authority_dimensions
        .iter()
        .filter(|requested| !resolved.contains(requested))
        .cloned()
        .collect();

    let valid = denied.is_empty();
    let mut reason_codes = vec![ReasonCode("AUTHORITY_RESOLVED_FROM_ROLE_AND_CONTEXT".into())];
    if valid {
        reason_codes.push(ReasonCode("REQUESTED_AUTHORITY_WITHIN_BOUNDARY".into()));
    } else {
        reason_codes.push(ReasonCode("REQUESTED_AUTHORITY_EXCEEDS_RESOLVED_BOUNDARY".into()));
    }

    AuthorityResolution {
        valid,
        resolved,
        denied,
        reason_codes,
    }
}

fn push_unique(target: &mut Vec<AuthorityDimension>, value: AuthorityDimension) {
    if !target.contains(&value) {
        target.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mizan_model::{RelationshipDomain, RouteKind};

    fn human_corrective() -> MizanInput {
        MizanInput {
            actor_id: "human".into(),
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
            evidence: vec![],
        }
    }

    #[test]
    fn corrective_human_gets_counsel_not_admin() {
        let r = resolve_authority(&human_corrective());
        assert!(r.valid);
        assert!(r.resolved.contains(&AuthorityDimension::Counsel));
        assert!(!r.resolved.contains(&AuthorityDimension::Admin));
    }

    #[test]
    fn corrective_human_cannot_request_admin_authority() {
        let mut input = human_corrective();
        input.requested_authority_dimensions.push(AuthorityDimension::Admin);
        let r = resolve_authority(&input);
        assert!(!r.valid);
        assert!(r.denied.contains(&AuthorityDimension::Admin));
    }
}
